#!/usr/bin/env python3
"""
Test client for MiniMax Proxy Server
Usage:
    python test_client.py "Your question here"
    python test_client.py --multi
"""

import sys
import argparse
from typing import Optional

try:
    import httpx
except ImportError:
    print("Installing httpx...")
    import subprocess
    subprocess.check_call([sys.executable, "-m", "pip", "install", "httpx"])
    import httpx


BASE_URL = "http://localhost:8080"


class ProxyError(Exception):
    """Base exception for proxy errors."""
    pass


class AuthenticationError(ProxyError):
    """Authentication required or expired."""
    pass


class QuotaError(ProxyError):
    """Quota exceeded."""
    pass


class ApiError(ProxyError):
    """General API error."""
    pass


def check_health() -> dict:
    """Check server health."""
    response = httpx.get(f"{BASE_URL}/health", timeout=10)
    return response.json()


def check_auth_status() -> dict:
    """Check authentication status."""
    response = httpx.get(f"{BASE_URL}/auth/status", timeout=10)
    if response.status_code == 401:
        raise AuthenticationError("Not authenticated. Run: curl -X POST http://localhost:8080/auth/login")
    return response.json()


def chat(
    messages: list[dict],
    model: Optional[str] = None,
    max_tokens: Optional[int] = None,
    temperature: Optional[float] = None,
) -> dict:
    """Send chat request to proxy."""
    payload = {"messages": messages}
    
    if model:
        payload["model"] = model
    if max_tokens:
        payload["max_tokens"] = max_tokens
    if temperature is not None:
        payload["temperature"] = temperature
    
    response = httpx.post(
        f"{BASE_URL}/v1/chat/completions",
        json=payload,
        timeout=60,
    )
    
    if response.status_code == 401:
        error = response.json()
        msg = error['error']['message']
        raise AuthenticationError(msg)
    
    if response.status_code != 200:
        error = response.json()
        msg = error['error']['message']
        code = error['error'].get('code', '')
        if code == 'QUOTA_EXCEEDED' or 'quota' in msg.lower() or 'balance' in msg.lower():
            raise QuotaError(msg)
        raise ApiError(msg)
    
    return response.json()


def login(region: str = "global") -> dict:
    """Trigger OAuth login flow."""
    response = httpx.post(
        f"{BASE_URL}/auth/login",
        headers={"X-Region": region},
        timeout=30,
    )
    return response.json()


def main():
    parser = argparse.ArgumentParser(description="Test MiniMax Proxy")
    parser.add_argument("question", nargs="?", help="Question to ask")
    parser.add_argument("--multi", action="store_true", help="Run multiple test questions")
    parser.add_argument("--model", default=None, help="Model to use")
    parser.add_argument("--region", default="global", help="Region for login (global or cn)")
    args = parser.parse_args()
    
    print("=" * 60)
    print("MiniMax Proxy Test Client")
    print("=" * 60)
    
    # Check health
    print("\n1. Checking server health...")
    try:
        health = check_health()
        print(f"   Health: {health}")
    except Exception as e:
        print(f"   ✗ Failed to connect: {e}")
        print(f"   Make sure the proxy server is running:")
        print(f"   cargo run")
        sys.exit(1)
    
    # Check auth
    print("\n2. Checking authentication...")
    try:
        auth = check_auth_status()
        print(f"   Auth: {auth}")
        if not auth.get("authenticated"):
            print("\n   ⚠ Not authenticated!")
    except AuthenticationError as e:
        print(f"   ✗ Not authenticated: {e}")
        print("\n   To login with OAuth:")
        print(f"   curl -X POST http://localhost:8080/auth/login")
        print(f"\n   Or use your browser to run:")
        print(f"   mmx auth login --recommend --region={args.region}")
        if not args.multi:
            sys.exit(1)
    except Exception as e:
        print(f"   ✗ Auth check failed: {e}")
        if not args.multi:
            sys.exit(1)
    
    if args.multi:
        # Run multiple test questions
        questions = [
            ("Simple greeting", [{"role": "user", "content": "Say hello in exactly 3 words"}]),
            ("Coding question", [{"role": "user", "content": "What is 2 + 2? Just give the number."}]),
            ("Technical question", [{"role": "user", "content": "Explain what a proxy server is in one sentence"}]),
        ]
        
        for i, (name, messages) in enumerate(questions, 1):
            print(f"\n{'=' * 60}")
            print(f"Test {i}: {name}")
            print("-" * 60)
            print(f"Question: {messages[0]['content']}")
            
            try:
                response = chat(messages, model=args.model)
                answer = response["choices"][0]["message"]["content"]
                print(f"\nAnswer: {answer}")
                print(f"Model: {response['model']}")
            except AuthenticationError as e:
                print(f"\n✗ Authentication error: {e}")
                print("   Run: curl -X POST http://localhost:8080/auth/login")
            except QuotaError as e:
                print(f"\n✗ Quota exceeded: {e}")
                print("   Your Code Plan has run out of quota.")
                print("   Check: mmx quota show")
            except ApiError as e:
                print(f"\n✗ API error: {e}")
            except Exception as e:
                print(f"\n✗ Error: {e}")
    
    elif args.question:
        # Single question
        print(f"\n3. Sending question...")
        print(f"   Q: {args.question}")
        
        messages = [{"role": "user", "content": args.question}]
        
        try:
            response = chat(messages, model=args.model)
            answer = response["choices"][0]["message"]["content"]
            print(f"\n   A: {answer}")
            print(f"\n   Model: {response['model']}")
            print(f"   Response ID: {response['id']}")
        except AuthenticationError as e:
            print(f"\n   ✗ Authentication error: {e}")
            print("   Run: curl -X POST http://localhost:8080/auth/login")
            sys.exit(1)
        except QuotaError as e:
            print(f"\n   ✗ Quota exceeded: {e}")
            print("   Your Code Plan has run out of quota.")
            sys.exit(1)
        except ApiError as e:
            print(f"\n   ✗ API error: {e}")
            sys.exit(1)
    
    else:
        # Default test
        question = "What is MiniMax? Answer in exactly 10 words or less."
        print(f"\n3. Default test question...")
        print(f"   Q: {question}")
        
        messages = [{"role": "user", "content": question}]
        
        try:
            response = chat(messages, model=args.model)
            answer = response["choices"][0]["message"]["content"]
            print(f"\n   A: {answer}")
            print(f"\n   Model: {response['model']}")
        except AuthenticationError as e:
            print(f"\n   ✗ Authentication error: {e}")
            sys.exit(1)
        except QuotaError as e:
            print(f"\n   ✗ Quota exceeded: {e}")
            sys.exit(1)
        except ApiError as e:
            print(f"\n   ✗ API error: {e}")
            sys.exit(1)
    
    print("\n" + "=" * 60)
    print("Done!")
    print("=" * 60)


if __name__ == "__main__":
    main()
