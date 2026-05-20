# Server

An Axum server that models after multi-tenant manufacturing + logistics, and explores a variety of distributed systems scenarios

## Distributed Manufacturing & Logistics Platform

A cloud-native manufacturing and supply chain operating system designed to explore distributed systems concepts through realistic backend infrastructure.

## Core Domains
- Production / MES
- Inventory & Warehousing
- Supply Chain & Procurement
- Transportation & Fleet
- Scheduling & Optimization
- Telemetry & Observability
- Simulation & Analytics
- Identity & Multi-Tenant Auth

## Key Entities
- Factory
- ProductionLine
- Machine
- Sensor
- WorkOrder
- Batch
- SKU
- InventoryReservation
- Warehouse
- Shipment
- Route
- Supplier
- PurchaseOrder
- TelemetryEvent
- DowntimeEvent
- ProductionSchedule
- SimulationRun

## Distributed Systems Concepts

### Consistency

- Inventory reservation races
- Optimistic concurrency
- Distributed locking
- Eventual consistency

### Messaging & Streams
- Event bus
- Append-only logs
- Consumer groups
- Replayable streams
- Dead letter queues

### Caching
- Cache-aside
- Write-through
- Stampede prevention
- Distributed cache invalidation

### Infrastructure
- Queue / Kafka-lite
- Cache / Redis-lite
- Scheduler
- TSDB
- Service discovery
- Consensus & replication

### Real-Time Systems
- Machine telemetry ingestion
- Live dashboards
- Backpressure handling
- Time-series aggregation

### Workflow Orchestration
- Long-running workflows
- Sagas
- Retries
- Idempotency
- Compensating transactions

## Build Order
- Modular monolith
- Event-driven architecture
- Internal queue + cache systems
- Distributed services
- Replication + consensus
- Simulation + analytics engine
