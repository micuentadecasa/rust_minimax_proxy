import { Annotation, END, START, StateGraph } from '@langchain/langgraph';

const DEFAULT_MODEL = 'MiniMax-M2.7';

export const AGENTS = {
  coordinator: {
    id: 'coordinator',
    name: 'Coordinator Agent',
    role: 'Reads the request and chooses the best specialist agent.',
    tools: ['Deterministic request router'],
  },
  prd: {
    id: 'prd',
    name: 'PRD Agent',
    role: 'Turns the user request into a product requirements document.',
    tools: ['MiniMax proxy chat completion'],
  },
  plan: {
    id: 'plan',
    name: 'Planning Agent',
    role: 'Turns the request or PRD into an implementation plan.',
    tools: ['MiniMax proxy chat completion'],
  },
  jobs: {
    id: 'jobs',
    name: 'Jobs Agent',
    role: 'Finds current Spain jobs in informatics, data, and artificial intelligence from UNJobNet.',
    tools: ['UNJobNet Spain Scrapy scraper service'],
  },
};

function emitStep(onStep, agentId, status, output = '', extra = {}) {
  const agent = AGENTS[agentId];
  onStep?.({
    agentId,
    agentName: agent.name,
    role: agent.role,
    tools: agent.tools,
    status,
    output,
    ...extra,
  });
}

const GraphState = Annotation.Root({
  userInput: Annotation(),
  proxyBaseUrl: Annotation(),
  jobsApiBaseUrl: Annotation(),
  model: Annotation(),
  fetchImpl: Annotation(),
  onStep: Annotation(),
  route: Annotation(),
  routeReason: Annotation(),
  prd: Annotation(),
  plan: Annotation(),
  jobs: Annotation(),
  jobsSource: Annotation(),
  output: Annotation(),
});

function trimBaseUrl(baseUrl) {
  return String(baseUrl || '').replace(/\/+$/, '');
}

export function routeAgentRequest(userInput) {
  const text = String(userInput || '').toLowerCase();
  const jobWords = [
    'job',
    'jobs',
    'position',
    'positions',
    'vacancy',
    'vacancies',
    'career',
    'careers',
    'work',
    'employment',
  ];
  const jobDomainWords = [
    'informatics',
    'information technology',
    'ict',
    'data',
    'analytics',
    'artificial intelligence',
    'ai',
    'machine learning',
    'software',
    'developer',
    'digital',
  ];
  const planWords = [
    'plan',
    'implementation',
    'implement',
    'phases',
    'tasks',
    'steps',
    'roadmap',
    'sprint',
    'milestone',
    'tests',
    'ticket',
  ];
  const prdWords = [
    'prd',
    'requirements',
    'requirement',
    'product spec',
    'specification',
    'user stories',
    'success metrics',
    'acceptance criteria',
    'problem',
    'goals',
  ];

  const wantsJobs = jobWords.some((word) => text.includes(word));
  const wantsJobDomain = jobDomainWords.some((word) => text.includes(word));
  const wantsPlan = planWords.some((word) => text.includes(word));
  const wantsPrd = prdWords.some((word) => text.includes(word));

  if (wantsJobs) {
    return {
      route: 'jobs',
      reason: wantsJobDomain
        ? 'The request asks for jobs and mentions informatics, data, artificial intelligence, or related technology terms.'
        : 'The request asks for jobs; the Jobs Agent will search UNJobNet Spain and filter for informatics, data, and artificial intelligence roles.',
    };
  }

  if (wantsPlan && !wantsPrd) {
    return {
      route: 'plan',
      reason: 'The request asks for implementation planning, phases, tasks, or engineering steps.',
    };
  }

  if (wantsPlan && wantsPrd) {
    const planIndex = Math.min(...planWords.map((word) => text.indexOf(word)).filter((idx) => idx >= 0));
    const prdIndex = Math.min(...prdWords.map((word) => text.indexOf(word)).filter((idx) => idx >= 0));
    if (planIndex < prdIndex) {
      return {
        route: 'plan',
        reason: 'The request mentions both planning and PRD terms, with planning as the primary ask.',
      };
    }
  }

  if (wantsPrd) {
    return {
      route: 'prd',
      reason: 'The request asks for product requirements, a PRD, specs, stories, or success metrics.',
    };
  }

  return {
    route: 'prd',
    reason: 'Default route: ambiguous product requests start with a PRD before implementation planning.',
  };
}

async function callJobsSearch({ jobsApiBaseUrl, query, limit = 10, fetchImpl = fetch }) {
  const url = new URL(`${trimBaseUrl(jobsApiBaseUrl || 'http://localhost:8090')}/jobs/search`);
  url.searchParams.set('q', query);
  url.searchParams.set('limit', String(limit));
  const response = await fetchImpl(url.toString(), { method: 'GET' });
  const payload = await response.json().catch(() => ({}));
  if (!response.ok) {
    throw new Error(payload?.detail || payload?.error?.message || `Jobs service returned HTTP ${response.status}`);
  }
  return {
    source: payload?.source || 'https://www.unjobnet.org/countries/Spain',
    jobs: Array.isArray(payload?.jobs) ? payload.jobs : [],
  };
}

async function callProxyChat({ proxyBaseUrl, model = DEFAULT_MODEL, messages, fetchImpl = fetch }) {
  const response = await fetchImpl(`${trimBaseUrl(proxyBaseUrl)}/v1/chat/completions`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({
      model,
      messages,
    }),
  });

  const payload = await response.json().catch(() => ({}));
  if (!response.ok) {
    throw new Error(payload?.error?.message || `Proxy returned HTTP ${response.status}`);
  }

  const text = payload?.choices?.[0]?.message?.content?.trim();
  if (!text) {
    throw new Error('Proxy response did not include text for the agent output.');
  }

  return text;
}

async function coordinatorAgent(state) {
  emitStep(state.onStep, 'coordinator', 'running');
  const decision = routeAgentRequest(state.userInput);
  const output = `Selected ${AGENTS[decision.route].name}. Reason: ${decision.reason}`;
  emitStep(state.onStep, 'coordinator', 'complete', output, {
    route: decision.route,
    routeReason: decision.reason,
  });
  return {
    route: decision.route,
    routeReason: decision.reason,
  };
}

async function prdAgent(state) {
  emitStep(state.onStep, 'prd', 'running');
  const prd = await callProxyChat({
    proxyBaseUrl: state.proxyBaseUrl,
    model: state.model,
    fetchImpl: state.fetchImpl,
    messages: [
      {
        role: 'system',
        content:
          'You are a senior product requirements agent. Turn the user request into a concise markdown product requirements document with sections: Problem, Users, Goals, Non-goals, Functional Requirements, Constraints, Success Metrics, and Open Questions. Do not write an implementation plan unless explicitly needed as a constraint.',
      },
      {
        role: 'user',
        content: `Create a product requirements document for this request:\n\n${state.userInput}`,
      },
    ],
  });

  emitStep(state.onStep, 'prd', 'complete', prd);
  return { prd, output: prd };
}

async function jobsAgent(state) {
  emitStep(state.onStep, 'jobs', 'running');
  const result = await callJobsSearch({
    jobsApiBaseUrl: state.jobsApiBaseUrl,
    query: state.userInput,
    limit: 10,
    fetchImpl: state.fetchImpl,
  });
  const output = result.jobs.length
    ? `Found ${result.jobs.length} matching UNJobNet Spain job${result.jobs.length === 1 ? '' : 's'}.`
    : 'No matching UNJobNet Spain jobs found for this request.';
  emitStep(state.onStep, 'jobs', 'complete', output, {
    jobs: result.jobs,
    jobsSource: result.source,
  });
  return { jobs: result.jobs, jobsSource: result.source, output };
}

async function planningAgent(state) {
  emitStep(state.onStep, 'plan', 'running');
  const prdContext = state.prd
    ? `PRD to plan from:\n${state.prd}`
    : 'No separate PRD was provided. Create the implementation plan directly from the user request and clearly state assumptions.';
  const plan = await callProxyChat({
    proxyBaseUrl: state.proxyBaseUrl,
    model: state.model,
    fetchImpl: state.fetchImpl,
    messages: [
      {
        role: 'system',
        content:
          'You are an implementation planner. Convert the request or PRD into a practical markdown implementation plan with phases, files/modules to touch, tests, risks, rollout, and done criteria.',
      },
      {
        role: 'user',
        content: `User request:\n${state.userInput}\n\n${prdContext}`,
      },
    ],
  });

  emitStep(state.onStep, 'plan', 'complete', plan);
  return { plan, output: plan };
}

export function createCoordinatorGraph() {
  return new StateGraph(GraphState)
    .addNode('coordinatorAgent', coordinatorAgent)
    .addNode('prdAgent', prdAgent)
    .addNode('planningAgent', planningAgent)
    .addNode('jobsAgent', jobsAgent)
    .addEdge(START, 'coordinatorAgent')
    .addConditionalEdges('coordinatorAgent', (state) => state.route, {
      prd: 'prdAgent',
      plan: 'planningAgent',
      jobs: 'jobsAgent',
    })
    .addEdge('prdAgent', END)
    .addEdge('planningAgent', END)
    .addEdge('jobsAgent', END)
    .compile();
}

export async function runCoordinatorGraph({
  userInput,
  proxyBaseUrl,
  jobsApiBaseUrl = 'http://localhost:8090',
  model = DEFAULT_MODEL,
  fetchImpl = fetch,
  onStep,
}) {
  const content = String(userInput || '').trim();
  if (!content) {
    throw new Error('User input is required to run the coordinator graph.');
  }

  const graph = createCoordinatorGraph();
  const result = await graph.invoke({
    userInput: content,
    proxyBaseUrl,
    jobsApiBaseUrl,
    model,
    fetchImpl,
    onStep,
  });

  return {
    route: result.route,
    routeReason: result.routeReason,
    prd: result.prd,
    plan: result.plan,
    jobs: result.jobs,
    jobsSource: result.jobsSource,
    output: result.output,
  };
}

export const createPrdPlanGraph = createCoordinatorGraph;
export const runPrdPlanGraph = runCoordinatorGraph;
