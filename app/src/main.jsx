import React, { useMemo, useState } from 'react';
import { createRoot } from 'react-dom/client';
import { CopilotKit } from '@copilotkit/react-core';
import { AGENTS, runPrdPlanGraph } from './agentGraph.js';
import '@copilotkit/react-ui/styles.css';
import './styles.css';

const PROXY_BASE_URL = import.meta.env.VITE_PROXY_BASE_URL || 'http://localhost:8080';
const DEFAULT_MODEL = import.meta.env.VITE_MINIMAX_MODEL || 'MiniMax-M2.7';

function agentStatusLabel(status) {
  if (status === 'complete') return 'Complete';
  if (status === 'running') return 'Running';
  return 'Waiting';
}

function ChatApp() {
  const [messages, setMessages] = useState([
    {
      role: 'assistant',
      content: 'Hi — describe a feature and I will run the PRD Agent first, then the Planning Agent. Normal chat is still available in the mode selector.',
    },
  ]);
  const [input, setInput] = useState('');
  const [mode, setMode] = useState('prd-plan');
  const [isLoading, setIsLoading] = useState(false);
  const [agentStep, setAgentStep] = useState('');
  const [agentTrace, setAgentTrace] = useState([]);
  const [artifacts, setArtifacts] = useState(null);
  const [error, setError] = useState('');

  const statusText = useMemo(() => {
    if (isLoading && mode === 'prd-plan') return agentStep || 'Running PRD → plan graph…';
    if (isLoading) return 'Waiting for MiniMax…';
    if (error) return 'Proxy error';
    return `Ready via ${PROXY_BASE_URL}`;
  }, [agentStep, error, isLoading, mode]);

  async function sendMessage(event) {
    event.preventDefault();
    const content = input.trim();
    if (!content || isLoading) return;

    const nextMessages = [...messages, { role: 'user', content }];
    setMessages(nextMessages);
    setInput('');
    setError('');
    setArtifacts(null);
    setAgentTrace([]);
    setAgentStep(mode === 'prd-plan' ? 'PRD Agent is starting…' : '');
    setIsLoading(true);

    try {
      if (mode === 'prd-plan') {
        const result = await runPrdPlanGraph({
          userInput: content,
          proxyBaseUrl: PROXY_BASE_URL,
          model: DEFAULT_MODEL,
          onStep: (event) => {
            setAgentStep(`${event.agentName}: ${agentStatusLabel(event.status)}`);
            setAgentTrace((current) => {
              const withoutCurrentAgent = current.filter((item) => item.agentId !== event.agentId);
              return [...withoutCurrentAgent, event];
            });
          },
        });
        setAgentStep('Planning Agent: Complete');
        setArtifacts(result);
        setMessages([
          ...nextMessages,
          {
            role: 'assistant',
            content: 'The PRD → plan graph finished. Review the product requirements and implementation plan below.',
          },
        ]);
        return;
      }

      const response = await fetch(`${PROXY_BASE_URL}/v1/chat/completions`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          model: DEFAULT_MODEL,
          messages: nextMessages.map(({ role, content: text }) => ({ role, content: text })),
        }),
      });

      const payload = await response.json().catch(() => ({}));
      if (!response.ok) {
        throw new Error(payload?.error?.message || `Proxy returned HTTP ${response.status}`);
      }

      const assistantText = payload?.choices?.[0]?.message?.content?.trim();
      if (!assistantText) {
        throw new Error('Proxy response did not include an assistant message.');
      }

      setMessages([...nextMessages, { role: 'assistant', content: assistantText }]);
    } catch (err) {
      const message = err instanceof Error ? err.message : String(err);
      setError(message);
      setMessages([...nextMessages, { role: 'assistant', content: `Error: ${message}` }]);
    } finally {
      setIsLoading(false);
      setAgentStep('');
    }
  }

  return (
    <main className="shell">
      <section className="hero" aria-labelledby="title">
        <p className="eyebrow">CopilotKit + MiniMax OAuth Proxy</p>
        <h1 id="title">MiniMax CopilotKit Chat</h1>
        <p className="subtitle">
          A small React chat surface that sends OpenAI-compatible requests to your local Rust proxy.
        </p>
        <div className="status" data-loading={isLoading} data-testid="status-text">
          {statusText}
        </div>
      </section>

      <section className="chat-card" aria-label="Chat conversation">
        <div className="messages" data-testid="message-list">
          {messages.map((message, index) => (
            <article
              className={`message ${message.role}`}
              data-testid={message.role === 'assistant' ? 'assistant-message' : 'user-message'}
              key={`${message.role}-${index}-${message.content.slice(0, 12)}`}
            >
              <span className="role">{message.role === 'assistant' ? 'Assistant' : 'You'}</span>
              <p>{message.content}</p>
            </article>
          ))}

          {mode === 'prd-plan' && agentTrace.length > 0 && (
            <section className="agent-trace" data-testid="agent-trace" aria-label="Agent execution trace">
              {[AGENTS.prd, AGENTS.plan].map((agent) => {
                const event = agentTrace.find((item) => item.agentId === agent.id);
                const output = event?.output || (agent.id === 'prd' ? artifacts?.prd : artifacts?.plan) || '';
                return (
                  <article
                    className={`agent-card ${event?.status || 'waiting'}`}
                    data-testid={agent.id === 'prd' ? 'agent-card-prd' : 'agent-card-plan'}
                    key={agent.id}
                  >
                    <header className="agent-card-header">
                      <div>
                        <span className="role">{agent.id === 'prd' ? 'First agent' : 'Second agent'}</span>
                        <h2>{agent.name}</h2>
                        <p>{agent.role}</p>
                      </div>
                      <strong>{agentStatusLabel(event?.status)}</strong>
                    </header>
                    <div className="tool-list">
                      <span className="role">Tools used</span>
                      {agent.tools.map((tool) => <code key={tool}>{tool}</code>)}
                    </div>
                    {output ? (
                      <div
                        className="agent-output"
                        data-testid={agent.id === 'prd' ? 'prd-output' : 'plan-output'}
                      >
                        <span className="role">Output</span>
                        <p>{output}</p>
                      </div>
                    ) : (
                      <p className="agent-waiting">Waiting for the previous agent to finish…</p>
                    )}
                  </article>
                );
              })}
            </section>
          )}

          {isLoading && (
            <article className="message assistant loading" data-testid="assistant-loading">
              <span className="role">{mode === 'prd-plan' ? 'Agent graph' : 'Assistant'}</span>
              <p>{mode === 'prd-plan' ? 'Running PRD Agent, then Planning Agent…' : 'Thinking through MiniMax…'}</p>
            </article>
          )}
        </div>

        {mode === 'prd-plan' && agentStep && (
          <div className="agent-step" data-testid="agent-step-status">
            {agentStep}
          </div>
        )}

        <form className="composer" onSubmit={sendMessage}>
          <div className="mode-row">
            <label htmlFor="mode">Mode</label>
            <select
              id="mode"
              value={mode}
              onChange={(event) => setMode(event.target.value)}
              disabled={isLoading}
              data-testid="mode-select"
            >
              <option value="chat">Normal chat</option>
              <option value="prd-plan">PRD → plan LangGraph</option>
            </select>
          </div>
          <label htmlFor="message">Message</label>
          <div className="composer-row">
            <textarea
              id="message"
              name="message"
              rows="3"
              value={input}
              onChange={(event) => setInput(event.target.value)}
              placeholder={mode === 'prd-plan' ? 'Describe the product or feature you want planned…' : 'Ask the proxy-backed assistant…' }
              disabled={isLoading}
            />
            <button type="submit" disabled={isLoading || input.trim().length === 0}>
              {isLoading ? (mode === 'prd-plan' ? 'Running…' : 'Sending…') : (mode === 'prd-plan' ? 'Run agent graph' : 'Send')}
            </button>
          </div>
        </form>

        {error && <p className="error" role="alert">{error}</p>}
      </section>
    </main>
  );
}

function Root() {
  return (
    <CopilotKit runtimeUrl={PROXY_BASE_URL} showDevConsole={false}>
      <ChatApp />
    </CopilotKit>
  );
}

createRoot(document.getElementById('root')).render(<Root />);
