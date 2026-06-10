import React, { useMemo, useState } from 'react';
import { createRoot } from 'react-dom/client';
import { CopilotKit } from '@copilotkit/react-core';
import { AGENTS, runCoordinatorGraph } from './agentGraph.js';
import '@copilotkit/react-ui/styles.css';
import './styles.css';

const PROXY_BASE_URL = import.meta.env.VITE_PROXY_BASE_URL || 'http://localhost:8080';
const JOBS_API_BASE_URL = import.meta.env.VITE_JOBS_API_BASE_URL || 'http://localhost:8090';
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
      content: 'Hi — describe what you need. The Coordinator Agent will choose the PRD Agent, Planning Agent, or Jobs Agent. Ask about Spain jobs in informatics, data, or artificial intelligence to search UNJobNet. Normal chat is still available in the mode selector.',
    },
  ]);
  const [input, setInput] = useState('');
  const [mode, setMode] = useState('agent-router');
  const [isLoading, setIsLoading] = useState(false);
  const [agentStep, setAgentStep] = useState('');
  const [agentTrace, setAgentTrace] = useState([]);
  const [artifacts, setArtifacts] = useState(null);
  const [error, setError] = useState('');

  const statusText = useMemo(() => {
    if (isLoading && mode === 'agent-router') return agentStep || 'Coordinator Agent is routing…';
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
    setAgentStep(mode === 'agent-router' ? 'Coordinator Agent is routing…' : '');
    setIsLoading(true);

    try {
      if (mode === 'agent-router') {
        const result = await runCoordinatorGraph({
          userInput: content,
          proxyBaseUrl: PROXY_BASE_URL,
          jobsApiBaseUrl: JOBS_API_BASE_URL,
          model: DEFAULT_MODEL,
          onStep: (event) => {
            setAgentStep(`${event.agentName}: ${agentStatusLabel(event.status)}`);
            setAgentTrace((current) => {
              const existingIndex = current.findIndex((item) => item.agentId === event.agentId);
              if (existingIndex === -1) return [...current, event];
              return current.map((item, index) => (index === existingIndex ? event : item));
            });
          },
        });
        const routedAgentName = result.route === 'plan' ? 'Planning Agent' : result.route === 'jobs' ? 'Jobs Agent' : 'PRD Agent';
        setAgentStep(`${routedAgentName}: Complete`);
        setArtifacts(result);
        setMessages([
          ...nextMessages,
          {
            role: 'assistant',
            content: `The Coordinator Agent routed this to the ${routedAgentName}. Review the answer below.`,
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

          {mode === 'agent-router' && agentTrace.length > 0 && (
            <section className="agent-trace" data-testid="agent-trace" aria-label="Agent execution trace">
              {agentTrace.map((event) => {
                const agent = AGENTS[event.agentId];
                const output = event?.output || (agent.id === 'prd' ? artifacts?.prd : agent.id === 'plan' ? artifacts?.plan : artifacts?.output) || '';
                const jobs = event?.jobs || (agent.id === 'jobs' ? artifacts?.jobs : null) || [];
                return (
                  <article
                    className={`agent-card ${event?.status || 'waiting'}`}
                    data-testid={`agent-card-${agent.id}`}
                    key={agent.id}
                  >
                    <header className="agent-card-header">
                      <div>
                        <span className="role">{agent.id === 'coordinator' ? 'Coordinator' : 'Selected specialist'}</span>
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
                        data-testid={agent.id === 'coordinator' ? 'coordinator-output' : agent.id === 'prd' ? 'prd-output' : agent.id === 'jobs' ? 'jobs-output' : 'plan-output'}
                      >
                        <span className="role">Output</span>
                        <p>{output}</p>
                        {agent.id === 'jobs' && jobs.length > 0 && (
                          <div className="job-results" aria-label="Matching jobs">
                            {jobs.map((job, jobIndex) => (
                              <article className="job-card" data-testid="job-card" key={`${job.url || job.title}-${jobIndex}`}>
                                <div>
                                  <h3 data-testid="job-title">{job.title || 'Untitled job'}</h3>
                                  {job.organization && <p data-testid="job-organization">{job.organization}</p>}
                                </div>
                                <dl>
                                  <div>
                                    <dt>Location</dt>
                                    <dd data-testid="job-location">{job.location || 'Spain'}</dd>
                                  </div>
                                  <div>
                                    <dt>Deadline</dt>
                                    <dd data-testid="job-deadline">{job.deadline || 'Not listed'}</dd>
                                  </div>
                                </dl>
                                {job.summary && <p className="job-summary">{job.summary}</p>}
                                {Array.isArray(job.matchedTerms) && job.matchedTerms.length > 0 && (
                                  <p className="job-terms">Search matched: {job.matchedTerms.join(', ')}</p>
                                )}
                                {Array.isArray(job.cvMatchedTerms) && job.cvMatchedTerms.length > 0 && (
                                  <p className="job-terms" data-testid="job-cv-match">
                                    CV fit ({job.cvMatchScore || 0}): {job.cvMatchedTerms.join(', ')}
                                  </p>
                                )}
                                {job.url && (
                                  <a data-testid="job-link" href={job.url} target="_blank" rel="noreferrer">
                                    View job
                                  </a>
                                )}
                              </article>
                            ))}
                          </div>
                        )}
                        {agent.id === 'jobs' && jobs.length === 0 && event?.status === 'complete' && (
                          <p className="jobs-empty" data-testid="jobs-empty">No matching UNJobNet Spain jobs found.</p>
                        )}
                      </div>
                    ) : (
                      <p className="agent-waiting">Waiting for this agent to finish…</p>
                    )}
                  </article>
                );
              })}
            </section>
          )}

          {isLoading && (
            <article className="message assistant loading" data-testid="assistant-loading">
              <span className="role">{mode === 'agent-router' ? 'Agent graph' : 'Assistant'}</span>
              <p>{mode === 'agent-router' ? 'Coordinator Agent is choosing a specialist…' : 'Thinking through MiniMax…'}</p>
            </article>
          )}
        </div>

        {mode === 'agent-router' && agentStep && (
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
              <option value="agent-router">Coordinator agent graph</option>
              <option value="chat">Normal chat</option>
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
              placeholder={mode === 'agent-router' ? 'Ask for a PRD, an implementation plan, or Spain data/AI jobs…' : 'Ask the proxy-backed assistant…' }
              disabled={isLoading}
            />
            <button type="submit" disabled={isLoading || input.trim().length === 0}>
              {isLoading ? (mode === 'agent-router' ? 'Routing…' : 'Sending…') : (mode === 'agent-router' ? 'Ask coordinator' : 'Send')}
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
