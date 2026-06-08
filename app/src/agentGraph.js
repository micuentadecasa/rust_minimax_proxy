import { Annotation, END, START, StateGraph } from '@langchain/langgraph';

const DEFAULT_MODEL = 'MiniMax-M2.7';

export const AGENTS = {
  prd: {
    id: 'prd',
    name: 'PRD Agent',
    role: 'Turns the user request into a product requirements document.',
    tools: ['MiniMax proxy chat completion'],
  },
  plan: {
    id: 'plan',
    name: 'Planning Agent',
    role: 'Turns the PRD into an implementation plan.',
    tools: ['MiniMax proxy chat completion'],
  },
};

function emitStep(onStep, agentId, status, output = '') {
  const agent = AGENTS[agentId];
  onStep?.({
    agentId,
    agentName: agent.name,
    role: agent.role,
    tools: agent.tools,
    status,
    output,
  });
}

const GraphState = Annotation.Root({
  userInput: Annotation(),
  proxyBaseUrl: Annotation(),
  model: Annotation(),
  fetchImpl: Annotation(),
  onStep: Annotation(),
  prd: Annotation(),
  plan: Annotation(),
});

function trimBaseUrl(baseUrl) {
  return String(baseUrl || '').replace(/\/+$/, '');
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
          'You are a senior product requirements agent. Turn the user request into a concise markdown product requirements document with sections: Problem, Users, Goals, Non-goals, Functional Requirements, Constraints, Success Metrics, and Open Questions.',
      },
      {
        role: 'user',
        content: `Create a product requirements document for this request:\n\n${state.userInput}`,
      },
    ],
  });

  emitStep(state.onStep, 'prd', 'complete', prd);
  return { prd };
}

async function planningAgent(state) {
  emitStep(state.onStep, 'plan', 'running');
  const plan = await callProxyChat({
    proxyBaseUrl: state.proxyBaseUrl,
    model: state.model,
    fetchImpl: state.fetchImpl,
    messages: [
      {
        role: 'system',
        content:
          'You are an implementation planner. Convert a PRD into a practical markdown implementation plan with phases, files/modules to touch, tests, risks, rollout, and done criteria.',
      },
      {
        role: 'user',
        content: `User request:\n${state.userInput}\n\nPRD to plan from:\n${state.prd}`,
      },
    ],
  });

  emitStep(state.onStep, 'plan', 'complete', plan);
  return { plan };
}

export function createPrdPlanGraph() {
  return new StateGraph(GraphState)
    .addNode('prdAgent', prdAgent)
    .addNode('planningAgent', planningAgent)
    .addEdge(START, 'prdAgent')
    .addEdge('prdAgent', 'planningAgent')
    .addEdge('planningAgent', END)
    .compile();
}

export async function runPrdPlanGraph({
  userInput,
  proxyBaseUrl,
  model = DEFAULT_MODEL,
  fetchImpl = fetch,
  onStep,
}) {
  const content = String(userInput || '').trim();
  if (!content) {
    throw new Error('User input is required to run the PRD/plan graph.');
  }

  const graph = createPrdPlanGraph();
  const result = await graph.invoke({
    userInput: content,
    proxyBaseUrl,
    model,
    fetchImpl,
    onStep,
  });

  return {
    prd: result.prd,
    plan: result.plan,
  };
}
