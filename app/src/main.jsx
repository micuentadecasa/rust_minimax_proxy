import React, { useMemo, useState } from 'react';
import { createRoot } from 'react-dom/client';
import { CopilotKit } from '@copilotkit/react-core';
import '@copilotkit/react-ui/styles.css';
import './styles.css';

const PROXY_BASE_URL = import.meta.env.VITE_PROXY_BASE_URL || 'http://localhost:8080';
const DEFAULT_MODEL = import.meta.env.VITE_MINIMAX_MODEL || 'MiniMax-M2.7';

function ChatApp() {
  const [messages, setMessages] = useState([
    {
      role: 'assistant',
      content: 'Hi — I am connected to your MiniMax proxy. Ask me anything.',
    },
  ]);
  const [input, setInput] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState('');

  const statusText = useMemo(() => {
    if (isLoading) return 'Waiting for MiniMax…';
    if (error) return 'Proxy error';
    return `Ready via ${PROXY_BASE_URL}`;
  }, [error, isLoading]);

  async function sendMessage(event) {
    event.preventDefault();
    const content = input.trim();
    if (!content || isLoading) return;

    const nextMessages = [...messages, { role: 'user', content }];
    setMessages(nextMessages);
    setInput('');
    setError('');
    setIsLoading(true);

    try {
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
          {isLoading && (
            <article className="message assistant loading" data-testid="assistant-loading">
              <span className="role">Assistant</span>
              <p>Thinking through MiniMax…</p>
            </article>
          )}
        </div>

        <form className="composer" onSubmit={sendMessage}>
          <label htmlFor="message">Message</label>
          <div className="composer-row">
            <textarea
              id="message"
              name="message"
              rows="3"
              value={input}
              onChange={(event) => setInput(event.target.value)}
              placeholder="Ask the proxy-backed assistant…"
              disabled={isLoading}
            />
            <button type="submit" disabled={isLoading || input.trim().length === 0}>
              {isLoading ? 'Sending…' : 'Send'}
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
