<template>
  <div class="summary-page">
    <!-- Header Component -->
    <header-component
        :userName="`Logged in as ${loggedUser}`"
        :showBackButton="true"
        @logout="handleLogout"
    />

    <!-- Summary Content -->
    <div class="summary-content">
      <div class="summary-container">

        <!-- Page Header with Navigation -->
        <div class="page-header">
          <div>
            <h1>Yearly Report</h1>
            <p class="header-subtitle">Last 12 Months Analysis</p>
          </div>
          <div class="header-nav">
            <router-link to="/summery-overview" class="nav-link">
              <i class="fa fa-calendar"></i>
              <span>Current Month</span>
            </router-link>
            <router-link to="/summery-vehicle" class="nav-link">
              <i class="fa fa-car"></i>
              <span>Vehicle Analytics</span>
            </router-link>
          </div>
        </div>

        <!-- === YEARLY OVERVIEW STATS === -->
        <div class="overview-stats">
          <div class="overview-stat">
            <div class="stat-label">Annual Income</div>
            <p class="stat-amount income">{{ formatPrice(yearlyIncome) }}</p>
            <div class="stat-change positive">
              <i class="fa fa-arrow-up"></i> +18.3% YoY
            </div>
          </div>
          <div class="overview-stat">
            <div class="stat-label">Annual Expenses</div>
            <p class="stat-amount expense">{{ formatPrice(yearlyExpenses) }}</p>
            <div class="stat-change negative">
              <i class="fa fa-arrow-up"></i> +8.5% YoY
            </div>
          </div>
          <div class="overview-stat">
            <div class="stat-label">Annual Profit</div>
            <p class="stat-amount" :class="{ 'negative': yearlyProfit < 0 }">
              {{ formatPrice(yearlyProfit) }}
            </p>
            <div class="stat-change" :class="{ 'negative': yearlyProfit < 0 }">
              <i :class="yearlyProfit >= 0 ? 'fa fa-arrow-up' : 'fa fa-arrow-down'"></i>
              {{ yearlyProfit >= 0 ? '+' : '' }}{{ yearlyMargin }}%
            </div>
          </div>
          <div class="overview-stat">
            <div class="stat-label">Profit Margin</div>
            <p class="stat-amount">{{ yearlyMargin }}%</p>
            <div class="stat-badge" :class="{ 'excellent': yearlyMargin >= 20, 'good': yearlyMargin >= 10 }">
              {{ yearlyMargin >= 20 ? 'Excellent' : yearlyMargin >= 10 ? 'Good' : 'Fair' }}
            </div>
          </div>
        </div>

        <!-- === YEARLY ANALYSIS SECTION === -->
        <div class="analysis-section">
          <div class="section-header">
            <div>
              <div class="section-title">12-Month Financial Breakdown</div>
              <p class="section-subtitle">Month-by-month performance</p>
            </div>
          </div>

          <!-- Tabs for Yearly -->
          <div class="tabs-container">
            <div class="tab-buttons">
              <button
                  class="tab-button"
                  :class="{ active: activeTab2 === 'graph' }"
                  @click="activeTab2 = 'graph'"
              >
                <i class="fa fa-chart-bar"></i>
                Graph View
              </button>
              <button
                  class="tab-button"
                  :class="{ active: activeTab2 === 'table' }"
                  @click="activeTab2 = 'table'"
              >
                <i class="fa fa-table"></i>
                Table View
              </button>
            </div>

            <!-- Tab 1: Graph View -->
            <div v-if="activeTab2 === 'graph'" class="tab-content graph-content">
              <div class="chart-container">
                <canvas id="yearlyChart"></canvas>
              </div>
            </div>

            <!-- Tab 2: Table View -->
            <div v-if="activeTab2 === 'table'" class="tab-content table-content">
              <div class="table-wrapper">
                <table class="analysis-table">
                  <thead>
                  <tr>
                    <th>Month</th>
                    <th>Income (LKR)</th>
                    <th>Expenses (LKR)</th>
                    <th>Net Profit (LKR)</th>
                    <th>Margin %</th>
                  </tr>
                  </thead>
                  <tbody>
                  <tr v-for="(month, index) in last12Months" :key="index">
                    <td class="month-cell">{{ month.name }}</td>
                    <td class="amount income">{{ formatPrice(month.income) }}</td>
                    <td class="amount expense">{{ formatPrice(month.expenses) }}</td>
                    <td class="amount" :class="{ 'negative': month.profit < 0 }">
                      {{ formatPrice(month.profit) }}
                    </td>
                    <td class="percentage" :class="{ 'negative': month.margin < 0 }">
                      <span class="margin-badge" :class="{ 'positive': month.margin >= 0 }">
                        {{ month.margin }}%
                      </span>
                    </td>
                  </tr>
                  <tr class="total-row">
                    <td><strong>Total</strong></td>
                    <td class="amount income"><strong>{{ formatPrice(yearlyIncome) }}</strong></td>
                    <td class="amount expense"><strong>{{ formatPrice(yearlyExpenses) }}</strong></td>
                    <td class="amount" :class="{ 'negative': yearlyProfit < 0 }">
                      <strong>{{ formatPrice(yearlyProfit) }}</strong>
                    </td>
                    <td class="percentage" :class="{ 'negative': yearlyMargin < 0 }">
                      <strong class="margin-badge" :class="{ 'positive': yearlyMargin >= 0 }">
                        {{ yearlyMargin }}%
                      </strong>
                    </td>
                  </tr>
                  </tbody>
                </table>
              </div>
            </div>
          </div>
        </div>

        <!-- === MONTHLY COMPARISON === -->
        <div class="comparison-section">
          <div class="section-header">
            <div>
              <div class="section-title">Best & Worst Performing Months</div>
              <p class="section-subtitle">Performance insights</p>
            </div>
          </div>

          <div class="comparison-grid">
            <div class="comparison-card best">
              <div class="card-icon">
                <i class="fa fa-trophy"></i>
              </div>
              <div class="card-content">
                <div class="card-label">Best Month</div>
                <div class="card-value">{{ bestMonth.name }}</div>
                <div class="card-detail">
                  <span class="detail-label">Profit:</span>
                  <span class="detail-value income">{{ formatPrice(bestMonth.profit) }}</span>
                </div>
                <div class="card-detail">
                  <span class="detail-label">Margin:</span>
                  <span class="detail-value">{{ bestMonth.margin }}%</span>
                </div>
              </div>
            </div>

            <div class="comparison-card worst">
              <div class="card-icon">
                <i class="fa fa-exclamation-circle"></i>
              </div>
              <div class="card-content">
                <div class="card-label">Lowest Month</div>
                <div class="card-value">{{ worstMonth.name }}</div>
                <div class="card-detail">
                  <span class="detail-label">Profit:</span>
                  <span class="detail-value negative">{{ formatPrice(worstMonth.profit) }}</span>
                </div>
                <div class="card-detail">
                  <span class="detail-label">Margin:</span>
                  <span class="detail-value">{{ worstMonth.margin }}%</span>
                </div>
              </div>
            </div>

            <div class="comparison-card average">
              <div class="card-icon">
                <i class="fa fa-bar-chart"></i>
              </div>
              <div class="card-content">
                <div class="card-label">Average Monthly</div>
                <div class="card-value">{{ (yearlyIncome / 12).toFixed(0) }}</div>
                <div class="card-detail">
                  <span class="detail-label">Avg Income:</span>
                  <span class="detail-value">{{ formatPrice(yearlyIncome / 12) }}</span>
                </div>
                <div class="card-detail">
                  <span class="detail-label">Avg Margin:</span>
                  <span class="detail-value">{{ averageMargin }}%</span>
                </div>
              </div>
            </div>
          </div>
        </div>

      </div>
    </div>
  </div>
</template>

<script>
import HeaderComponent from '../component/Header.vue';
import { dbService } from '../services/db.ts';
import Chart from 'chart.js/auto';

export default {
  name: 'SummaryYearly',
  components: {
    HeaderComponent
  },
  data() {
    return {
      loggedUser: 'Admin User',
      activeTab2: 'graph',
      accountSummery: [],
      charts: {}
    };
  },

  computed: {
    last12Months() {
      const months = [];
      for (let i = 11; i >= 0; i--) {
        const date = new Date();
        date.setMonth(date.getMonth() - i);

        const monthStart = new Date(date.getFullYear(), date.getMonth(), 1);
        const monthEnd = new Date(date.getFullYear(), date.getMonth() + 1, 0);

        const income_cash = this.accountSummery
            .filter(t => new Date(t.date) >= monthStart && new Date(t.date) <= monthEnd)
            .reduce((sum, t) => sum + t.income_cash, 0);

        const income_credit = this.accountSummery
            .filter(t => new Date(t.date) >= monthStart && new Date(t.date) <= monthEnd)
            .reduce((sum, t) => sum + t.income_credit, 0);

        const expenses_cash = this.accountSummery
            .filter(t => new Date(t.date) >= monthStart && new Date(t.date) <= monthEnd)
            .reduce((sum, t) => sum + t.expenses_cash, 0);

        const expenses_credit = this.accountSummery
            .filter(t => new Date(t.date) >= monthStart && new Date(t.date) <= monthEnd)
            .reduce((sum, t) => sum + t.expenses_credit, 0);

        const income = income_cash + income_credit;
        const expenses = expenses_cash + expenses_credit;
        const profit = income - expenses;
        const margin = income === 0 ? 0 : Math.round((profit / income) * 100);

        months.push({
          name: date.toLocaleString('en-US', { month: 'short', year: '2-digit' }),
          income,
          expenses,
          profit,
          margin
        });
      }
      return months;
    },

    yearlyIncome() {
      return this.last12Months.reduce((sum, m) => sum + m.income, 0);
    },

    yearlyExpenses() {
      return this.last12Months.reduce((sum, m) => sum + m.expenses, 0);
    },

    yearlyProfit() {
      return this.yearlyIncome - this.yearlyExpenses;
    },

    yearlyMargin() {
      if (this.yearlyIncome === 0) return 0;
      return Math.round((this.yearlyProfit / this.yearlyIncome) * 100);
    },

    bestMonth() {
      return this.last12Months.reduce((best, current) =>
          current.profit > best.profit ? current : best
      );
    },

    worstMonth() {
      return this.last12Months.reduce((worst, current) =>
          current.profit < worst.profit ? current : worst
      );
    },

    averageMargin() {
      const sum = this.last12Months.reduce((total, m) => total + m.margin, 0);
      return Math.round(sum / 12);
    }
  },

  methods: {
    formatPrice(price) {
      return new Intl.NumberFormat('en-US', {
        style: 'currency',
        currency: 'LKR',
        minimumFractionDigits: 2,
        maximumFractionDigits: 2
      }).format(price || 0);
    },

    handleLogout() {
      this.$emit('logout');
    },

    initializeYearlyChart() {
      const ctx = document.getElementById('yearlyChart');
      if (!ctx) return;

      if (this.charts.yearly) {
        this.charts.yearly.destroy();
      }

      const labels = this.last12Months.map(m => m.name);
      const incomeData = this.last12Months.map(m => m.income);
      const expenseData = this.last12Months.map(m => m.expenses);

      this.charts.yearly = new Chart(ctx, {
        type: 'bar',
        data: {
          labels,
          datasets: [
            {
              label: 'Income',
              data: incomeData,
              backgroundColor: '#22c55e',
              borderRadius: 6,
              borderSkipped: false
            },
            {
              label: 'Expenses',
              data: expenseData,
              backgroundColor: '#ef4444',
              borderRadius: 6,
              borderSkipped: false
            }
          ]
        },
        options: {
          responsive: true,
          maintainAspectRatio: true,
          plugins: {
            legend: {
              position: 'top',
              labels: {
                usePointStyle: true,
                padding: 20,
                font: { size: 13, weight: '600' },
                color: 'var(--color-text-primary)'
              }
            }
          },
          scales: {
            y: {
              beginAtZero: true,
              grid: {
                color: 'var(--color-border-tertiary)',
                drawBorder: false
              },
              ticks: {
                color: 'var(--color-text-secondary)',
                font: { size: 12 }
              }
            },
            x: {
              grid: {
                display: false
              },
              ticks: {
                color: 'var(--color-text-secondary)',
                font: { size: 12 }
              }
            }
          }
        }
      });
    },

    async getData() {
      try {
        this.accountSummery = await dbService.getAccountSummery();

        this.$nextTick(() => {
          this.initializeYearlyChart();
        });
      } catch (error) {
        console.log('Error fetching data:', error);
      }
    }
  },

  async mounted() {
    await this.getData();
  }
};
</script>

<style scoped>
.summary-page {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
  background: var(--color-background-tertiary);
}

.summary-content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.summary-container {
  display: flex;
  flex-direction: column;
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem;
  max-width: 1400px;
  margin: 0 auto;
  width: 100%;
}

/* === PAGE HEADER === */
.page-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 2rem;
  gap: 2rem;
  flex-wrap: wrap;
}

.page-header h1 {
  margin: 0;
  font-size: 28px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.header-subtitle {
  margin: 0.5rem 0 0 0;
  font-size: 13px;
  color: var(--color-text-secondary);
  font-weight: 500;
}

.header-nav {
  display: flex;
  gap: 1rem;
}

.nav-link {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.25rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border-radius: 8px;
  text-decoration: none;
  font-size: 13px;
  font-weight: 600;
  transition: all 0.3s ease;
  white-space: nowrap;
}

.nav-link:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
}

/* === OVERVIEW STATS === */
.overview-stats {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1.5rem;
  margin-bottom: 2rem;
}

.overview-stat {
  background: var(--color-background-primary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-lg);
  padding: 1.5rem;
  transition: all 0.2s ease;
}

.overview-stat:hover {
  border-color: var(--color-border-secondary);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
}

.stat-label {
  font-size: 12px;
  color: var(--color-text-secondary);
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.3px;
  margin-bottom: 0.75rem;
}

.stat-amount {
  margin: 0;
  font-size: 28px;
  font-weight: 700;
  color: var(--color-text-primary);
  font-family: monospace;
  margin-bottom: 0.75rem;
}

.stat-amount.income {
  color: #22c55e;
}

.stat-amount.expense {
  color: #ef4444;
}

.stat-amount.negative {
  color: #ef4444;
}

.stat-change {
  display: inline-flex;
  align-items: center;
  gap: 0.4rem;
  font-size: 12px;
  font-weight: 600;
  padding: 0.4rem 0.75rem;
  border-radius: 4px;
  background: rgba(34, 197, 94, 0.1);
  color: #16a34a;
}

.stat-change.negative {
  background: rgba(239, 68, 68, 0.1);
  color: #dc2626;
}

.stat-badge {
  display: inline-block;
  font-size: 12px;
  font-weight: 600;
  padding: 0.4rem 0.75rem;
  border-radius: 4px;
  background: rgba(59, 130, 246, 0.1);
  color: #1e40af;
  margin-top: 0.5rem;
}

.stat-badge.excellent {
  background: rgba(34, 197, 94, 0.1);
  color: #16a34a;
}

.stat-badge.good {
  background: rgba(245, 158, 11, 0.1);
  color: #b45309;
}

/* === ANALYSIS SECTION === */
.analysis-section {
  background: var(--color-background-primary);
  border-radius: var(--border-radius-lg);
  border: 1px solid var(--color-border-tertiary);
  padding: 1.5rem;
  margin-bottom: 2rem;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 1.5rem;
  gap: 1rem;
}

.section-header > div:first-child {
  flex: 1;
}

.section-title {
  font-size: 16px;
  font-weight: 700;
  color: var(--color-text-primary);
  margin: 0 0 0.5rem 0;
}

.section-subtitle {
  font-size: 13px;
  color: var(--color-text-secondary);
  margin: 0;
  font-weight: 500;
}

/* === TABS === */
.tabs-container {
  margin-top: 1.5rem;
}

.tab-buttons {
  display: flex;
  gap: 1rem;
  margin-bottom: 1.5rem;
  border-bottom: 2px solid var(--color-border-tertiary);
}

.tab-button {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1rem;
  background: transparent;
  border: none;
  border-bottom: 2px solid transparent;
  color: var(--color-text-secondary);
  cursor: pointer;
  font-size: 14px;
  font-weight: 600;
  transition: all 0.3s ease;
  margin-bottom: -2px;
}

.tab-button:hover {
  color: var(--color-text-primary);
}

.tab-button.active {
  color: var(--color-info);
  border-bottom-color: var(--color-info);
}

.tab-content {
  animation: fadeIn 0.3s ease;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }
  to {
    opacity: 1;
  }
}

/* === CHARTS === */
.chart-container {
  background: var(--color-background-secondary);
  padding: 1.5rem;
  border-radius: 8px;
  border: 1px solid var(--color-border-tertiary);
}

#yearlyChart {
  max-height: 400px;
}

/* === TABLES === */
.table-wrapper {
  overflow-x: auto;
}

.analysis-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
  margin-top: 1rem;
}

.analysis-table thead {
  background: var(--color-background-secondary);
  border-bottom: 2px solid var(--color-border-tertiary);
}

.analysis-table th {
  padding: 1rem;
  text-align: left;
  font-weight: 600;
  color: var(--color-text-secondary);
  text-transform: capitalize;
}

.analysis-table td {
  padding: 1rem;
  border-bottom: 1px solid var(--color-border-tertiary);
  vertical-align: middle;
}

.analysis-table tbody tr:hover {
  background: var(--color-background-secondary);
}

.month-cell {
  font-weight: 600;
  color: var(--color-text-primary);
}

.amount {
  text-align: right;
  font-family: monospace;
  font-weight: 700;
}

.amount.income {
  color: #22c55e;
}

.amount.expense {
  color: #ef4444;
}

.amount.negative {
  color: #ef4444;
}

.percentage {
  text-align: center;
  font-weight: 600;
  color: var(--color-text-secondary);
}

.margin-badge {
  display: inline-block;
  padding: 0.35rem 0.65rem;
  border-radius: 4px;
  background: rgba(59, 130, 246, 0.1);
  color: #1e40af;
  font-weight: 700;
}

.margin-badge.positive {
  background: rgba(34, 197, 94, 0.1);
  color: #16a34a;
}

.margin-badge.negative {
  background: rgba(239, 68, 68, 0.1);
  color: #dc2626;
}

.total-row {
  background: var(--color-background-secondary);
  font-weight: 700;
  border-top: 2px solid var(--color-border-tertiary);
}

/* === COMPARISON SECTION === */
.comparison-section {
  background: var(--color-background-primary);
  border-radius: var(--border-radius-lg);
  border: 1px solid var(--color-border-tertiary);
  padding: 1.5rem;
  margin-bottom: 2rem;
}

.comparison-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 1.5rem;
  margin-top: 1.5rem;
}

.comparison-card {
  background: var(--color-background-secondary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: 12px;
  padding: 1.5rem;
  display: flex;
  gap: 1rem;
  transition: all 0.3s ease;
}

.comparison-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);
}

.comparison-card.best {
  border-left: 4px solid #22c55e;
}

.comparison-card.worst {
  border-left: 4px solid #ef4444;
}

.comparison-card.average {
  border-left: 4px solid #3b82f6;
}

.card-icon {
  width: 56px;
  height: 56px;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 24px;
  flex-shrink: 0;
}

.comparison-card.best .card-icon {
  background: rgba(34, 197, 94, 0.1);
  color: #22c55e;
}

.comparison-card.worst .card-icon {
  background: rgba(239, 68, 68, 0.1);
  color: #ef4444;
}

.comparison-card.average .card-icon {
  background: rgba(59, 130, 246, 0.1);
  color: #3b82f6;
}

.card-content {
  flex: 1;
}

.card-label {
  font-size: 12px;
  color: var(--color-text-secondary);
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.3px;
  margin-bottom: 0.4rem;
}

.card-value {
  font-size: 18px;
  font-weight: 700;
  color: var(--color-text-primary);
  margin-bottom: 0.8rem;
}

.card-detail {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem 0;
  border-top: 1px solid var(--color-border-tertiary);
  font-size: 12px;
}

.card-detail:first-of-type {
  border-top: none;
  padding-top: 0;
}

.detail-label {
  color: var(--color-text-secondary);
  font-weight: 600;
}

.detail-value {
  font-weight: 700;
  font-family: monospace;
  color: var(--color-text-primary);
}

.detail-value.income {
  color: #22c55e;
}

.detail-value.negative {
  color: #ef4444;
}

/* === RESPONSIVE === */
@media (max-width: 1024px) {
  .page-header {
    flex-direction: column;
  }

  .header-nav {
    width: 100%;
    flex-direction: column;
  }

  .nav-link {
    justify-content: center;
  }

  .overview-stats {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 768px) {
  .summary-container {
    padding: 1rem;
  }

  .page-header h1 {
    font-size: 22px;
  }

  .overview-stats {
    grid-template-columns: 1fr;
    gap: 1rem;
  }

  .stat-amount {
    font-size: 20px;
  }

  .comparison-grid {
    grid-template-columns: 1fr;
  }

  .analysis-table {
    font-size: 12px;
  }

  .analysis-table th,
  .analysis-table td {
    padding: 0.75rem;
  }
}

/* Scrollbar styling */
.summary-container::-webkit-scrollbar,
.table-wrapper::-webkit-scrollbar {
  width: 6px;
}

.summary-container::-webkit-scrollbar-track,
.table-wrapper::-webkit-scrollbar-track {
  background: transparent;
}

.summary-container::-webkit-scrollbar-thumb,
.table-wrapper::-webkit-scrollbar-thumb {
  background: var(--color-border-secondary);
  border-radius: 3px;
}

.summary-container::-webkit-scrollbar-thumb:hover,
.table-wrapper::-webkit-scrollbar-thumb:hover {
  background: var(--color-border-primary);
}
</style>
