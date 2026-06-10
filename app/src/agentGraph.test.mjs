import test from 'node:test';
import assert from 'node:assert/strict';
import { routeAgentRequest, runCoordinatorGraph } from './agentGraph.js';

function makeJsonResponse(body, status = 200) {
  return {
    ok: status >= 200 && status < 300,
    status,
    async json() {
      return body;
    },
  };
}

test('coordinator routes PRD requests to the PRD Agent only', async () => {
  const calls = [];
  const events = [];
  const fetchImpl = async (url, options) => {
    const body = JSON.parse(options.body);
    calls.push({ url, body });
    assert.match(body.messages[0].content, /product requirements/i);
    assert.match(body.messages.at(-1).content, /analytics dashboard/i);
    return makeJsonResponse({
      choices: [{ message: { content: '# PRD\nUsers need analytics dashboards.' } }],
    });
  };

  const result = await runCoordinatorGraph({
    userInput: 'Write a PRD for an analytics dashboard',
    proxyBaseUrl: 'http://proxy.test',
    model: 'MiniMax-M2.7',
    fetchImpl,
    onStep: (event) => events.push(event),
  });

  assert.equal(calls.length, 1);
  assert.equal(result.route, 'prd');
  assert.equal(result.output, '# PRD\nUsers need analytics dashboards.');
  assert.equal(result.prd, '# PRD\nUsers need analytics dashboards.');
  assert.equal(result.plan, undefined);
  assert.deepEqual(events.map((event) => event.agentId), ['coordinator', 'coordinator', 'prd', 'prd']);
  assert.equal(events[1].route, 'prd');
});

test('coordinator routes plan requests to the Planning Agent only', async () => {
  const calls = [];
  const events = [];
  const fetchImpl = async (url, options) => {
    const body = JSON.parse(options.body);
    calls.push({ url, body });
    assert.match(body.messages[0].content, /implementation planner/i);
    assert.match(body.messages.at(-1).content, /saved views/i);
    assert.doesNotMatch(body.messages.at(-1).content, /PRD to plan from:\s*undefined/i);
    return makeJsonResponse({
      choices: [{ message: { content: '# Plan\n1. Build saved views.' } }],
    });
  };

  const result = await runCoordinatorGraph({
    userInput: 'Create an implementation plan for saved views',
    proxyBaseUrl: 'http://proxy.test',
    fetchImpl,
    onStep: (event) => events.push(event),
  });

  assert.equal(calls.length, 1);
  assert.equal(result.route, 'plan');
  assert.equal(result.output, '# Plan\n1. Build saved views.');
  assert.equal(result.prd, undefined);
  assert.equal(result.plan, '# Plan\n1. Build saved views.');
  assert.deepEqual(events.map((event) => event.agentId), ['coordinator', 'coordinator', 'plan', 'plan']);
  assert.equal(events[1].route, 'plan');
});

test('coordinator routes informatics data and AI job requests to the Jobs Agent', async () => {
  const calls = [];
  const events = [];
  const fetchImpl = async (url, options = {}) => {
    calls.push({ url, options });
    assert.match(String(url), /\/jobs\/search\?/);
    assert.match(String(url), /q=.*data/i);
    return makeJsonResponse({
      source: 'https://www.unjobnet.org/countries/Spain',
      query: 'Find Spain data and AI jobs',
      jobs: [
        {
          title: 'Data Analyst',
          organization: 'UNICEF',
          location: 'Madrid, Spain',
          deadline: '2026-06-30',
          url: 'https://www.unjobnet.org/jobs/detail/1',
          summary: 'Analyze humanitarian data.',
          matchedTerms: ['data'],
        },
      ],
    });
  };

  const result = await runCoordinatorGraph({
    userInput: 'Find Spain data and AI jobs',
    proxyBaseUrl: 'http://proxy.test',
    jobsApiBaseUrl: 'http://jobs.test',
    fetchImpl,
    onStep: (event) => events.push(event),
  });

  assert.equal(calls.length, 1);
  assert.equal(result.route, 'jobs');
  assert.equal(result.jobs.length, 1);
  assert.equal(result.jobs[0].title, 'Data Analyst');
  assert.deepEqual(events.map((event) => event.agentId), ['coordinator', 'coordinator', 'jobs', 'jobs']);
});

test('routeAgentRequest routes generic job requests to the Jobs Agent', () => {
  const route = routeAgentRequest('show me the latest jobs in Spain');
  assert.equal(route.route, 'jobs');
  assert.match(route.reason, /jobs/i);
});

test('routeAgentRequest defaults ambiguous requests to PRD Agent', () => {
  const route = routeAgentRequest('Build saved dashboards');
  assert.equal(route.route, 'prd');
  assert.match(route.reason, /default/i);
});

test('agentGraph surfaces proxy failures', async () => {
  await assert.rejects(
    runCoordinatorGraph({
      userInput: 'write a plan',
      proxyBaseUrl: 'http://proxy.test',
      fetchImpl: async () => makeJsonResponse({ error: { message: 'token expired' } }, 401),
    }),
    /token expired/,
  );
});

test('agentGraph rejects empty model output', async () => {
  await assert.rejects(
    runCoordinatorGraph({
      userInput: 'write a PRD',
      proxyBaseUrl: 'http://proxy.test',
      fetchImpl: async () => makeJsonResponse({ choices: [{ message: { content: '   ' } }] }),
    }),
    /did not include text/,
  );
});
