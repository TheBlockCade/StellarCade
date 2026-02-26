const { evaluateMonitoringAlerts } = require('../utils/contractMonitoringAlerts');

const INITIAL_METRICS = Object.freeze({
  totalEvents: 0,
  settlementSuccess: 0,
  settlementFailed: 0,
  errorEvents: 0,
  pausedEvents: 0,
});

class ContractMonitoringService {
  constructor() {
    this.metrics = { ...INITIAL_METRICS };
    this.paused = false;
    this.seenEventIds = new Set();
  }

  ingestEvent({ eventId, kind }) {
    if (this.seenEventIds.has(eventId)) {
      throw new Error(`Duplicate event id: ${eventId}`);
    }

    this.seenEventIds.add(eventId);
    this.metrics.totalEvents += 1;

    switch (kind) {
      case 'settlement_success':
        this.metrics.settlementSuccess += 1;
        break;
      case 'settlement_failed':
        this.metrics.settlementFailed += 1;
        break;
      case 'error':
        this.metrics.errorEvents += 1;
        break;
      case 'paused':
        this.metrics.pausedEvents += 1;
        break;
      default:
        break;
    }

    return {
      metrics: this.getMetrics(),
      alerts: this.getAlerts(),
    };
  }

  setPaused(paused) {
    this.paused = Boolean(paused);
    return this.getHealth();
  }

  getMetrics() {
    return { ...this.metrics };
  }

  getAlerts() {
    return evaluateMonitoringAlerts(
      {
        totalEvents: this.metrics.totalEvents,
        settlementFailed: this.metrics.settlementFailed,
        errorEvents: this.metrics.errorEvents,
      },
      this.paused,
    );
  }

  getHealth() {
    return {
      status: this.paused ? 'paused' : 'running',
      alerts: this.getAlerts(),
      metrics: this.getMetrics(),
    };
  }
}

module.exports = new ContractMonitoringService();
