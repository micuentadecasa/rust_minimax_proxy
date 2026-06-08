# Fixed Schema A2UI

> Pre-defined A2UI schema with dynamic data. The fastest approach — no LLM schema generation needed.


<!-- interactive demo: a2ui-fixed-schema -->


In the fixed-schema approach, you design the UI schema once (by hand,
or using the [A2UI Composer](https://a2ui-composer.ag-ui.com/)) and
keep it on the agent side. The agent tool only provides the *data*;
the surface appears instantly when the tool returns because nothing
has to be generated at runtime.

How the schema is *delivered* to the runtime is the only thing that
varies between integrations:

- **Schema-loading** (langgraph-python, langgraph-typescript,
  langgraph-fastapi, llamaindex, crewai-crews, pydantic-ai,
  ms-agent-python, google-adk) — the schema is saved as a `.json`
  file next to the agent and loaded once at startup.
- **Schema-inline** (spring-ai, ms-agent-dotnet) — the schema is
  declared inline as a typed literal in source. The host language
  doesn't ship a `load_schema` JSON loader, so the structure is
  compiled in directly.
- **LLM-driven** (mastra, strands) — the agent runs a secondary LLM
  call to produce the operations container per-request. The catalog
  is still fixed; the schema is generated on demand.

Ask about a flight and the agent renders a fully structured card from a pre-defined schema:

## How it works

<FrameworkSetup concept="agent-setup" />

1. The schema is made available to the agent — either loaded from a
   JSON file at startup, declared inline, or generated per-request,
   depending on the integration.
2. The agent's `display_flight` tool receives data from the primary LLM
   (origin / destination / airline / price).
3. The tool returns `a2ui.render(...)` with `createSurface` +
   `updateComponents` + `updateDataModel` operations.
4. The A2UI middleware intercepts the tool result and the frontend
   renders the surface using the matching 5-component client catalog
   (Title, Airport, Arrow, AirlineBadge, PriceTag, plus the built-ins).

## Compositional schemas

The example below ships a flight card assembled compositionally from
small sub-components rather than one monolithic `FlightCard`:

```
Card
 └─ Column
     ├─ Title        ("Flight Details")
     ├─ Row          (Airport → Arrow → Airport)
     ├─ Row          (AirlineBadge · PriceTag)
     └─ Button       (Book)
```

That tree lives backend-side — as a JSON file, an inline literal, or
a per-request LLM output, depending on the integration. Components
without data bindings (like `Title` or `Arrow`) carry their value
inline; components bound to the LLM's data (like `Airport`) reference
fields via JSON Pointer paths such as `{ "path": "/origin" }`. The
A2UI binder resolves those paths *before* the React renderer runs, so
renderer props are typed as their resolved values (plain `z.string()`,
not a path-or-literal union).

## The 5-component custom catalog

The frontend catalog declares just the domain-specific primitives
(Title, Airport, Arrow, AirlineBadge, PriceTag) and merges in
CopilotKit's basic catalog (Card, Column, Row, Text, Button, …) via
`includeBasicCatalog: true`.

<Steps>
<Step>
### Declare the component definitions

Each component declares its props as a Zod schema. Props are the
*resolved* values, never the path expressions:

```typescript
// src/app/demos/a2ui-fixed-schema/a2ui/definitions.ts
import { z } from "zod";
import type { CatalogDefinitions } from "@copilotkit/a2ui-renderer";

/**
 * Dynamic string: literal OR a data-model path binding. The GenericBinder
 * resolves path bindings to the actual value at render time.
 */
const DynString = z.union([z.string(), z.object({ path: z.string() })]);

export const definitions = {
  /**
   * Card override: gives the outer flight-card container a ShadCN look
   * (rounded-xl, neutral-200 border, soft shadow). The basic catalog's
   * Card uses inline styles; overriding here lets the demo's renderer
   * adopt the demo's Tailwind aesthetic without touching the schema JSON.
   */
  Card: {
    description: "A container card with a single child.",
    props: z.object({
      child: z.string(),
    }),
  },
  Title: {
    description: "A prominent heading for the flight card.",
    props: z.object({
      text: DynString,
    }),
  },
  Airport: {
    description: "A 3-letter airport code, displayed large.",
    props: z.object({
      code: DynString,
    }),
  },
  Arrow: {
    description: "A right-pointing arrow used between airports.",
    props: z.object({}),
  },
  AirlineBadge: {
    description: "A pill-styled airline name tag.",
    props: z.object({
      name: DynString,
    }),
  },
  PriceTag: {
    description: "A stylized price display (e.g. '$289').",
    props: z.object({
      amount: DynString,
    }),
  },
  /**
   * Button override: swaps in an ActionButton renderer that tracks
   * its own `done` state so clicking "Book flight" visually updates to
   * a "Booked ✓" confirmation. The basic catalog's Button is stateless,
   * so without this override the click fires the action but the button
   * looks unchanged. Mirrors the pattern in beautiful-chat
   * (src/app/demos/beautiful-chat/declarative-generative-ui/renderers.tsx).
   */
  Button: {
    description:
      "An interactive button with an action event. Use 'child' with a Text component ID for the label. After click, the button shows a confirmation state.",
    props: z.object({
      child: z
        .string()
        .describe(
          "The ID of the child component (e.g. a Text component for the label).",
        ),
      variant: z.enum(["primary", "secondary", "ghost"]).optional(),
      // Union with { event } so GenericBinder resolves this as ACTION → callable () => void.
      action: z
        .union([
          z.object({
            event: z.object({
              name: z.string(),
              context: z.record(z.any()).optional(),
            }),
          }),
          z.null(),
        ])
        .optional(),
    }),
  },
} satisfies CatalogDefinitions;
```
</Step>

<Step>
### Implement the React renderers

TypeScript enforces that the renderer map's keys and prop shapes
match the definitions exactly, so refactors stay safe:

```typescript
// src/app/demos/a2ui-fixed-schema/a2ui/renderers.tsx
export const renderers: CatalogRenderers<Definitions> = {
  /**
   * Card override: ShadCN-style outer container. The basic catalog's Card
   * uses inline styles; overriding here keeps the demo's tailwind aesthetic.
   * The flight schema renders Card > Column > [Title, Row, …]; the inner
   * Column adds the vertical spacing.
   */
  Card: ({ props, children }) => (
    <Card className="w-full max-w-md p-5" data-testid="a2ui-fixed-card">
      {props.child ? children(props.child) : null}
    </Card>
  ),
  Title: ({ props }) => (
    <div className="flex items-center justify-between">
      <div className="space-y-1">
        <p className="text-[11px] font-medium uppercase tracking-[0.14em] text-neutral-500">
          Itinerary
        </p>
        <h3 className="text-base font-semibold leading-none tracking-tight text-neutral-900">
          {s(props.text)}
        </h3>
      </div>
      <Badge variant="outline" className="font-mono">
        1-stop · economy
      </Badge>
    </div>
  ),
  Airport: ({ props }) => (
    <div className="flex flex-col items-center">
      <span className="font-mono text-2xl font-semibold tracking-wider text-neutral-900">
        {s(props.code)}
      </span>
    </div>
  ),
  Arrow: () => (
    <div className="flex flex-1 items-center px-3">
      <Separator className="flex-1 bg-neutral-200" />
      <svg
        width="16"
        height="16"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
        className="mx-1 text-neutral-400"
        aria-hidden
      >
        <line x1="5" y1="12" x2="19" y2="12" />
        <polyline points="12 5 19 12 12 19" />
      </svg>
      <Separator className="flex-1 bg-neutral-200" />
    </div>
  ),
  AirlineBadge: ({ props }) => (
    <Badge variant="secondary" className="uppercase tracking-[0.08em]">
      {s(props.name)}
    </Badge>
  ),
  PriceTag: ({ props }) => (
    <div className="flex items-baseline gap-1">
      <span className="text-[11px] font-medium uppercase tracking-[0.14em] text-neutral-500">
        Total
      </span>
      <span className="font-mono text-base font-semibold text-neutral-900">
        {s(props.amount)}
      </span>
    </div>
  ),
  /**
   * Button override: this is a pure-presentation demo, so the button just
   * renders its label. The schema declares an `action` for visual fidelity,
   * but the click handler is inert until the Python SDK exposes
   * `action_handlers=` on `a2ui.render` (see `src/agents/a2ui_fixed.py`).
   */
  Button: ({ props, children }) => (
    <UIButton className="w-full">
      {props.child ? children(props.child) : null}
    </UIButton>
  ),
};
```
</Step>

<Step>
### Wire the catalog

`createCatalog(..., { includeBasicCatalog: true })` merges the custom
renderers with CopilotKit's built-ins so the schema can reference
`Card`, `Column`, `Row`, `Button` alongside the domain primitives:

```typescript
// src/app/demos/a2ui-fixed-schema/a2ui/catalog.ts
import { createCatalog } from "@copilotkit/a2ui-renderer";

import { definitions } from "./definitions";
import { renderers } from "./renderers";

export const CATALOG_ID = "copilotkit://flight-fixed-catalog";

export const catalog = createCatalog(definitions, renderers, {
  catalogId: CATALOG_ID,
  includeBasicCatalog: true,
});
```
</Step>

<WhenFrameworkHas flag="a2ui_pattern" equals="schema-loading">
<Step>
### Load the schema JSON at startup

`a2ui.load_schema(path)` (or the framework's equivalent thin `json.load`
wrapper) parses the schema file once at module-import time. The
sibling `booked_schema.json` is kept ready for the button-click
"booked" optimistic swap (see the note on action handlers below):

```python
# src/agents/a2ui_fixed.py
from __future__ import annotations

from pathlib import Path
from typing import TypedDict

from copilotkit import CopilotKitMiddleware, a2ui
from langchain.agents import create_agent
from langchain.tools import tool
from langchain_openai import ChatOpenAI

CATALOG_ID = "copilotkit://flight-fixed-catalog"
SURFACE_ID = "flight-fixed-schema"

_SCHEMAS_DIR = Path(__file__).parent / "a2ui_schemas"

# The schema is JSON so it can be authored and reviewed independently of the
# Python code. `a2ui.load_schema` is just a thin `json.load` wrapper.
FLIGHT_SCHEMA = a2ui.load_schema(_SCHEMAS_DIR / "flight_schema.json")
```
</Step>

<Step>
### Return render operations from the tool

The agent tool returns `a2ui.render(operations=[…])`. The A2UI
middleware detects the operations container in the tool result and
forwards it to the frontend renderer. The LLM only generates the four
data fields (`origin`, `destination`, `airline`, `price`); the schema
does the rest:

```python
# src/agents/a2ui_fixed.py
from __future__ import annotations

from pathlib import Path
from typing import TypedDict

from copilotkit import CopilotKitMiddleware, a2ui
from langchain.agents import create_agent
from langchain.tools import tool
from langchain_openai import ChatOpenAI

CATALOG_ID = "copilotkit://flight-fixed-catalog"
SURFACE_ID = "flight-fixed-schema"

_SCHEMAS_DIR = Path(__file__).parent / "a2ui_schemas"

# The schema is JSON so it can be authored and reviewed independently of the
# Python code. `a2ui.load_schema` is just a thin `json.load` wrapper.
FLIGHT_SCHEMA = a2ui.load_schema(_SCHEMAS_DIR / "flight_schema.json")


class Flight(TypedDict):
    """Shape the LLM should fill in when calling `display_flight`.

    LangGraph serializes this TypedDict into the tool's JSON schema, so
    defining it narrowly is how we steer the LLM to produce data that fits
    the frontend `FlightCard` component's props.
    """

    origin: str
    destination: str
    airline: str
    price: str


@tool
def display_flight(origin: str, destination: str, airline: str, price: str) -> str:
    """Show a flight card for the given trip.

    Use short airport codes (e.g. "SFO", "JFK") for origin/destination and a
    price string like "$289".

    After this tool returns, the flight card is already rendered to the user
    via the A2UI surface — the JSON returned here is the surface descriptor
    the renderer consumes, NOT a status code. Do NOT call this tool again
    for the same flight (the user already sees the card). Reply with one
    short confirmation sentence and stop.
    """
    # The A2UI middleware detects the `a2ui_operations` container in this
    # tool result and forwards the ops to the frontend renderer. The frontend
    # catalog resolves component names to the local React components.
    #
    # Note: schema-swap-on-action (e.g. swapping to a "booked" schema when
    # the card's button is clicked) will be added once the Python SDK
    # exposes `action_handlers=` on `a2ui.render`.
    return a2ui.render(
        operations=[
            a2ui.create_surface(SURFACE_ID, catalog_id=CATALOG_ID),
            a2ui.update_components(SURFACE_ID, FLIGHT_SCHEMA),
            a2ui.update_data_model(
                SURFACE_ID,
                {
                    "origin": origin,
                    "destination": destination,
                    "airline": airline,
                    "price": price,
                },
            ),
        ],
    )
```
</Step>
</WhenFrameworkHas>

<WhenFrameworkHas flag="a2ui_pattern" equals="schema-inline">
<Step>
### Define the schema inline

Spring AI / .NET don't ship a `load_schema` JSON helper, so the
component tree is declared inline as a typed literal in source —
equivalent to deserialising a `flight_schema.json` but compiled into
the agent class. The structure is identical to the JSON form; only
the surface syntax changes:

```python
# src/agents/a2ui_fixed.py
from __future__ import annotations

from pathlib import Path
from typing import TypedDict

from copilotkit import CopilotKitMiddleware, a2ui
from langchain.agents import create_agent
from langchain.tools import tool
from langchain_openai import ChatOpenAI

CATALOG_ID = "copilotkit://flight-fixed-catalog"
SURFACE_ID = "flight-fixed-schema"

_SCHEMAS_DIR = Path(__file__).parent / "a2ui_schemas"

# The schema is JSON so it can be authored and reviewed independently of the
# Python code. `a2ui.load_schema` is just a thin `json.load` wrapper.
FLIGHT_SCHEMA = a2ui.load_schema(_SCHEMAS_DIR / "flight_schema.json")
```
</Step>

<Step>
### Return render operations from the tool

The agent tool builds the same `createSurface` + `updateComponents` +
`updateDataModel` operations container and returns it. The A2UI
middleware detects the operations in the tool result and forwards
them to the frontend renderer; the LLM only supplies the four data
fields:

```python
# src/agents/a2ui_fixed.py
from __future__ import annotations

from pathlib import Path
from typing import TypedDict

from copilotkit import CopilotKitMiddleware, a2ui
from langchain.agents import create_agent
from langchain.tools import tool
from langchain_openai import ChatOpenAI

CATALOG_ID = "copilotkit://flight-fixed-catalog"
SURFACE_ID = "flight-fixed-schema"

_SCHEMAS_DIR = Path(__file__).parent / "a2ui_schemas"

# The schema is JSON so it can be authored and reviewed independently of the
# Python code. `a2ui.load_schema` is just a thin `json.load` wrapper.
FLIGHT_SCHEMA = a2ui.load_schema(_SCHEMAS_DIR / "flight_schema.json")


class Flight(TypedDict):
    """Shape the LLM should fill in when calling `display_flight`.

    LangGraph serializes this TypedDict into the tool's JSON schema, so
    defining it narrowly is how we steer the LLM to produce data that fits
    the frontend `FlightCard` component's props.
    """

    origin: str
    destination: str
    airline: str
    price: str


@tool
def display_flight(origin: str, destination: str, airline: str, price: str) -> str:
    """Show a flight card for the given trip.

    Use short airport codes (e.g. "SFO", "JFK") for origin/destination and a
    price string like "$289".

    After this tool returns, the flight card is already rendered to the user
    via the A2UI surface — the JSON returned here is the surface descriptor
    the renderer consumes, NOT a status code. Do NOT call this tool again
    for the same flight (the user already sees the card). Reply with one
    short confirmation sentence and stop.
    """
    # The A2UI middleware detects the `a2ui_operations` container in this
    # tool result and forwards the ops to the frontend renderer. The frontend
    # catalog resolves component names to the local React components.
    #
    # Note: schema-swap-on-action (e.g. swapping to a "booked" schema when
    # the card's button is clicked) will be added once the Python SDK
    # exposes `action_handlers=` on `a2ui.render`.
    return a2ui.render(
        operations=[
            a2ui.create_surface(SURFACE_ID, catalog_id=CATALOG_ID),
            a2ui.update_components(SURFACE_ID, FLIGHT_SCHEMA),
            a2ui.update_data_model(
                SURFACE_ID,
                {
                    "origin": origin,
                    "destination": destination,
                    "airline": airline,
                    "price": price,
                },
            ),
        ],
    )
```
</Step>
</WhenFrameworkHas>

<WhenFrameworkHas flag="a2ui_pattern" equals="llm-driven">
<Step>
### Generate the schema dynamically

Mastra and Strands take a different route: the agent tool runs a
*secondary* LLM call with a forced tool choice that produces the
operations container per-request. The frontend catalog is still fixed
(same `Title`/`Airport`/`Arrow`/`AirlineBadge`/`PriceTag` primitives),
but the schema is built on the fly. Schema construction and render
emission happen in the same tool call:

```python
# src/agents/a2ui_fixed.py
from __future__ import annotations

from pathlib import Path
from typing import TypedDict

from copilotkit import CopilotKitMiddleware, a2ui
from langchain.agents import create_agent
from langchain.tools import tool
from langchain_openai import ChatOpenAI

CATALOG_ID = "copilotkit://flight-fixed-catalog"
SURFACE_ID = "flight-fixed-schema"

_SCHEMAS_DIR = Path(__file__).parent / "a2ui_schemas"

# The schema is JSON so it can be authored and reviewed independently of the
# Python code. `a2ui.load_schema` is just a thin `json.load` wrapper.
FLIGHT_SCHEMA = a2ui.load_schema(_SCHEMAS_DIR / "flight_schema.json")


class Flight(TypedDict):
    """Shape the LLM should fill in when calling `display_flight`.

    LangGraph serializes this TypedDict into the tool's JSON schema, so
    defining it narrowly is how we steer the LLM to produce data that fits
    the frontend `FlightCard` component's props.
    """

    origin: str
    destination: str
    airline: str
    price: str


@tool
def display_flight(origin: str, destination: str, airline: str, price: str) -> str:
    """Show a flight card for the given trip.

    Use short airport codes (e.g. "SFO", "JFK") for origin/destination and a
    price string like "$289".

    After this tool returns, the flight card is already rendered to the user
    via the A2UI surface — the JSON returned here is the surface descriptor
    the renderer consumes, NOT a status code. Do NOT call this tool again
    for the same flight (the user already sees the card). Reply with one
    short confirmation sentence and stop.
    """
    # The A2UI middleware detects the `a2ui_operations` container in this
    # tool result and forwards the ops to the frontend renderer. The frontend
    # catalog resolves component names to the local React components.
    #
    # Note: schema-swap-on-action (e.g. swapping to a "booked" schema when
    # the card's button is clicked) will be added once the Python SDK
    # exposes `action_handlers=` on `a2ui.render`.
    return a2ui.render(
        operations=[
            a2ui.create_surface(SURFACE_ID, catalog_id=CATALOG_ID),
            a2ui.update_components(SURFACE_ID, FLIGHT_SCHEMA),
            a2ui.update_data_model(
                SURFACE_ID,
                {
                    "origin": origin,
                    "destination": destination,
                    "airline": airline,
                    "price": price,
                },
            ),
        ],
    )
```
</Step>
</WhenFrameworkHas>
</Steps>

## Why compositional beats monolithic

A single big `FlightCard` component would be faster to write but would
lock the design in place. Assembling the card from Card / Column /
Row / Title / Airport / Arrow / AirlineBadge / PriceTag gives you:

- **Reusable primitives** — the same `Airport` renderer works in
  search results, booking confirmations, and future seat maps.
- **Schema-level design iteration** — re-arranging rows or swapping a
  badge requires only a JSON edit; the renderer code is untouched.
- **A2UI Composer compatibility** — hand-written and Composer-built
  schemas share the same primitive vocabulary.

## Registering the runtime

On the TypeScript side, A2UI's middleware auto-detects the operations
in any tool result, so even with a fixed schema, the minimum setup
is `a2ui: {}`. The `a2ui-fixed-schema` cell happens to also keep
`injectA2UITool: true` so the same agent can be pointed at
dynamic-schema workflows later without re-configuring.

```typescript title="app/api/copilotkit/route.ts"
const runtime = new CopilotRuntime({
  agents: { "a2ui-fixed-schema": agent },
  a2ui: { injectA2UITool: true, agents: ["a2ui-fixed-schema"] },
});
```

## Action handlers (reference)

The canonical reference pairs fixed schemas with
`action_handlers={...}` to declare optimistic UI swaps (e.g. replacing
the flight schema with `BOOKED_SCHEMA` when the user clicks "Book").
The Python SDK's `a2ui.render` does not yet accept `action_handlers`,
so the cell omits them; the `booked_schema.json` sibling is retained
so the swap can be wired up the moment the SDK exposes the handler
kwarg.

When available, a button declares its action like this:

```json
{
  "Button": {
    "label": "Book",
    "action": {
      "name": "book_flight",
      "context": [
        { "key": "flightNumber", "value": { "path": "/flightNumber" } },
        { "key": "price", "value": { "path": "/price" } }
      ]
    }
  }
}
```

And the Python tool matches it with a handler keyed by the action
name (plus a `"*"` catch-all). Until the SDK lands, see the reference
[fixed-schema guide](/integrations/langgraph/generative-ui/a2ui/fixed-schema)
for the full pattern.

## When should I use fixed schemas?

- The surface is well-known: flight cards, product tiles, order
  summaries, dashboards.
- You want deterministic, designer-controlled UI. No LLM schema drift.
- You want the fastest possible first paint; no secondary LLM call.

If the UI must adapt per prompt, reach for
**[dynamic schemas](./dynamic-schema)** instead.

<IntegrationGrid path="generative-ui/a2ui" />
