import test from 'node:test';
import assert from 'node:assert/strict';
import { runPrdPlanGraph } from './agentGraph.js';

function makeJsonResponse(body, status = 200) {
  return {
    ok: status >= 200 && status < 300,
    status,
    async json() {
      return body;
    },
  };
}

test('agentGraph runs the PRD agent before the planning agent', async () => {
  const calls = [];
  const events = [];
  const fetchImpl = async (url, options) => {
    const body = JSON.parse(options.body);
    calls.push({ url, body });

    if (calls.length === 1) {
      assert.match(body.messages[0].content, /product requirements/i);
      assert.match(body.messages.at(-1).content, /Build saved dashboards/i);
      return makeJsonResponse({
        choices: [{ message: { content: '# PRD\nUsers need saved dashboards.' } }],
      });
    }

    assert.match(body.messages[0].content, /implementation planner/i);
    assert.match(body.messages.at(-1).content, /# PRD/);
    return makeJsonResponse({
      choices: [{ message: { content: '# Plan\n1. Add dashboard persistence.' } }],
    });
  };

  const result = await runPrdPlanGraph({
    userInput: 'Build saved dashboards',
    proxyBaseUrl: 'http://proxy.test',
    model: 'MiniMax-M2.7',
    fetchImpl,
    onStep: (event) => events.push(event),
  });

  assert.equal(calls.length, 2);
  assert.equal(calls[0].url, 'http://proxy.test/v1/chat/completions');
  assert.equal(result.prd, '# PRD\nUsers need saved dashboards.');
  assert.equal(result.plan, '# Plan\n1. Add dashboard persistence.');
  assert.deepEqual(events.map((event) => event.agentId), [
    'prd',
    'prd',
    'plan',
    'plan',
  ]);
  assert.deepEqual(events.map((event) => event.status), [
    'running',
    'complete',
    'running',
    'complete',
  ]);
  assert.equal(events[0].agentName, 'PRD Agent');
  assert.deepEqual(events[0].tools, ['MiniMax proxy chat completion']);
});

test('agentGraph surfaces proxy failures', async () => {
  await assert.rejects(
    runPrdPlanGraph({
      userInput: 'anything',
      proxyBaseUrl: 'http://proxy.test',
      fetchImpl: async () => makeJsonResponse({ error: { message: 'token expired' } }, 401),
    }),
    /token expired/,
  );
});

test('agentGraph rejects empty model output', async () => {
  await assert.rejects(
    runPrdPlanGraph({
      userInput: 'anything',
      proxyBaseUrl: 'http://proxy.test',
      fetchImpl: async () => makeJsonResponse({ choices: [{ message: { content: '   ' } }] }),
    }),
    /did not include text/,
  );
});
