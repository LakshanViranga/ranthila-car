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

        <!-- Page Header -->
        <div class="page-header">
          <h1>Financial Summary & Analytics</h1>
          <div class="header-date">{{ currentDate }}</div>
        </div>

        <!-- === SECTION 1: CURRENT MONTH ANALYSIS === -->
        <div class="analysis-section">
          <div class="section-title">Current Month Analysis</div>

          <!-- Summary Stats -->
          <div class="stats-overview">
            <div class="stat-item">
              <label>Total Income</label>
              <p class="stat-value income">{{ formatPrice(currentMonthIncome) }}</p>
            </div>
            <div class="stat-item">
              <label>Total Expenses</label>
              <p class="stat-value expense">{{ formatPrice(currentMonthExpenses) }}</p>
            </div>
            <div class="stat-item">
              <label>Net Profit</label>
              <p class="stat-value" :class="{ 'negative': currentMonthProfit < 0 }">
                {{ formatPrice(currentMonthProfit) }}
              </p>
            </div>
            <div class="stat-item">
              <label>Profit Margin</label>
              <p class="stat-value">{{ profitMargin }}%</p>
            </div>
          </div>

          <!-- Tabs for Current Month -->
          <div class="tabs-container">
            <div class="tab-buttons">
              <button
                  class="tab-button"
                  :class="{ active: activeTab1 === 'graph' }"
                  @click="activeTab1 = 'graph'"
              >
                <i class="fa fa-chart-line"></i>
                Graph View
              </button>
              <button
                  class="tab-button"
                  :class="{ active: activeTab1 === 'table' }"
                  @click="activeTab1 = 'table'"
              >
                <i class="fa fa-table"></i>
                Table View
              </button>
            </div>

            <!-- Tab 1: Graph View -->
            <div v-if="activeTab1 === 'graph'" class="tab-content graph-content">
              <div class="chart-container">
                <canvas id="currentMonthChart"></canvas>
              </div>
            </div>

            <!-- Tab 2: Table View -->
            <div v-if="activeTab1 === 'table'" class="tab-content table-content">
              <div class="table-wrapper">
                <table class="analysis-table">
                  <thead>
                  <tr>
                    <th>Category</th>
                    <th>Amount (LKR)</th>
                    <th>Percentage</th>
                    <th>Chart</th>
                  </tr>
                  </thead>
                  <tbody>
                  <tr>
                    <td class="category-cell income">
                      <i class="fa fa-arrow-down"></i> Total Income
                    </td>
                    <td class="amount income">{{ formatPrice(currentMonthIncome) }}</td>
                    <td class="percentage">100%</td>
                    <td class="chart-cell">
                      <div class="mini-bar income" style="width: 100%"></div>
                    </td>
                  </tr>
                  <tr>
                    <td class="category-cell expense">
                      <i class="fa fa-arrow-up"></i> Total Expenses
                    </td>
                    <td class="amount expense">{{ formatPrice(currentMonthExpenses) }}</td>
                    <td class="percentage">{{ expensePercentageOfIncome }}%</td>
                    <td class="chart-cell">
                      <div class="mini-bar expense" :style="{ width: expensePercentageOfIncome + '%' }"></div>
                    </td>
                  </tr>
                  <tr class="total-row">
                    <td class="category-cell">
                      <i class="fa fa-chart-line"></i> Net Profit
                    </td>
                    <td class="amount" :class="{ 'negative': currentMonthProfit < 0 }">
                      {{ formatPrice(currentMonthProfit) }}
                    </td>
                    <td class="percentage profit">{{ profitMargin }}%</td>
                    <td class="chart-cell">
                      <div class="mini-bar profit" :style="{ width: profitMargin + '%' }"></div>
                    </td>
                  </tr>
                  </tbody>
                </table>
              </div>
            </div>
          </div>

          <!-- Pending Credit Settlements -->
          <div class="pending-settlements-section">
            <div class="section-header">
              <div>
                <div class="section-title">
                  <i class="fa fa-hourglass-half"></i>
                  Current Month Pending Settlements
                </div>
                <p class="section-subtitle">Expenses awaiting payment settlement</p>
              </div>
              <div class="settlement-badge" v-if="expensesRecords.length > 0">
                {{ expensesRecords.length }} pending
              </div>
            </div>

            <!-- Settlements List -->
            <div class="settlements-container">
              <div
                  v-if="expensesRecords.length > 0"
                  class="settlements-grid"
              >
                <div
                    v-for="(amount, index) in expensesRecords"
                    :key="index"
                    class="settlement-card"
                >
                  <div class="settlement-content">
                    <div class="settlement-left">
                      <div class="settlement-icon">
                        <i class="fa fa-receipt"></i>
                      </div>
                      <div class="settlement-info">
                        <span class="settlement-label">Settlement</span>
                        <span class="settlement-id">#{{ index + 1 }}</span>
                      </div>
                    </div>
                    <div class="settlement-amount">
                      {{ formatPrice(amount) }}
                    </div>
                  </div>
                  <div class="settlement-footer">
                     <button class="status-badge pending">
                        <i class="fa fa-check-circle-o"></i> Settle
                     </button>
                  </div>
                </div>
              </div>

              <!-- Empty State -->
              <div v-else class="empty-settlements">
                <div class="empty-icon">
                  <i class="fa fa-check-circle"></i>
                </div>
                <p class="empty-title">All Settled</p>
                <p class="empty-message">No pending settlements for this month</p>
              </div>
            </div>
          </div>
        </div>

        <!-- === SECTION 2: YEARLY OVERVIEW === -->
        <div class="analysis-section">
          <div class="section-title">12-Month Overview (Last Year)</div>

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
                      {{ month.margin }}%
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
                      <strong>{{ yearlyMargin }}%</strong>
                    </td>
                  </tr>
                  </tbody>
                </table>
              </div>
            </div>
          </div>
        </div>

        <!-- === SECTION 3: VEHICLE-WISE ANALYSIS === -->
        <div class="analysis-section">
          <div class="section-title">Vehicle-wise Income & Expenses Analysis</div>

          <!-- Vehicle Filter -->
          <div class="filter-section">
<!--            <input-->
<!--                v-model="vehicleSearchQuery"-->
<!--                type="text"-->
<!--                class="search-input"-->
<!--                placeholder="Search vehicle..."-->
<!--            />-->
            <select
                id="vehicle"
                v-model="vehicleSearchQuery"
                class="input-field"
                required
            >
              <option value="">Select vehicle</option>
              <option v-for="vehicle in vehicles" :key="vehicle.vehicle_id" :value="vehicle.vehicle_id">
                {{ vehicle.registerNumber }} - {{ vehicle.manufacturer }} {{ vehicle.modelName }}
              </option>
            </select>
          </div>

          <!-- Vehicle Cards Grid -->
          <div class="vehicle-grid">
            <div
                v-for="vehicle in filteredVehicles"
                :key="vehicle.id"
                class="vehicle-card"
                @click="selectedVehicleAnalysis = vehicle"
            >
              <div class="vehicle-header">
                <h3>{{ vehicle.registerNumber }}</h3>
                <span class="vehicle-name">{{ vehicle.manufacturer }} {{ vehicle.modelName }}</span>
              </div>
              <div class="vehicle-stats">
                <div class="vehicle-stat">
                  <label>Income</label>
                  <p class="income">{{ formatPrice(vehicle.totalIncome) }}</p>
                </div>
                <div class="vehicle-stat">
                  <label>Expenses</label>
                  <p class="expense">{{ formatPrice(vehicle.totalExpenses) }}</p>
                </div>
                <div class="vehicle-stat">
                  <label>Net Profit</label>
                  <p :class="{ 'negative': vehicle.netProfit < 0 }">
                    {{ formatPrice(vehicle.netProfit) }}
                  </p>
                </div>
              </div>
              <div class="vehicle-footer">
                <span class="profit-margin" :class="{ 'positive': vehicle.profitMargin >= 0 }">
                  {{ vehicle.profitMargin }}% margin
                </span>
              </div>
            </div>
          </div>

          <!-- Empty State -->
          <div v-if="filteredVehicles.length === 0" class="empty-state">
            <i class="fa fa-car"></i>
            <p>No vehicles found</p>
          </div>
        </div>

        <!-- === VEHICLE DETAIL MODAL === -->
        <div v-if="selectedVehicleAnalysis" class="modal-overlay" @click="closeVehicleDetail">
          <div class="modal-content" @click.stop>
            <div class="modal-header">
              <div>
                <h2>{{ selectedVehicleAnalysis.registerNumber }}</h2>
                <p class="vehicle-name">{{ selectedVehicleAnalysis.manufacturer }} {{ selectedVehicleAnalysis.modelName }}</p>
              </div>
              <button class="btn-close" @click="selectedVehicleAnalysis = null">
                <i class="fa fa-times"></i>
              </button>
            </div>

            <div class="modal-body">
              <!-- Vehicle Summary Stats -->
              <div class="vehicle-summary">
                <div class="summary-item">
                  <label>Total Income</label>
                  <p class="income">{{ formatPrice(selectedVehicleAnalysis.totalIncome) }}</p>
                </div>
                <div class="summary-item">
                  <label>Total Expenses</label>
                  <p class="expense">{{ formatPrice(selectedVehicleAnalysis.totalExpenses) }}</p>
                </div>
                <div class="summary-item">
                  <label>Net Profit</label>
                  <p :class="{ 'negative': selectedVehicleAnalysis.netProfit < 0 }">
                    {{ formatPrice(selectedVehicleAnalysis.netProfit) }}
                  </p>
                </div>
                <div class="summary-item">
                  <label>Profit Margin</label>
                  <p>{{ selectedVehicleAnalysis.profitMargin }}%</p>
                </div>
              </div>

              <!-- Vehicle Tabs -->
              <div class="tabs-container">
                <div class="tab-buttons">
                  <button
                      class="tab-button small"
                      :class="{ active: activeVehicleTab === 'income' }"
                      @click="activeVehicleTab = 'income'"
                  >
                    <i class="fa fa-arrow-down"></i>
                    Income Details
                  </button>
                  <button
                      class="tab-button small"
                      :class="{ active: activeVehicleTab === 'expenses' }"
                      @click="activeVehicleTab = 'expenses'"
                  >
                    <i class="fa fa-arrow-up"></i>
                    Expense Details
                  </button>
                  <button
                      class="tab-button small"
                      :class="{ active: activeVehicleTab === 'breakdown' }"
                      @click="activeVehicleTab = 'breakdown'"
                  >
                    <i class="fa fa-chart-pie"></i>
                    Breakdown
                  </button>
                </div>

                <!-- Income Details -->
                <div v-if="activeVehicleTab === 'income'" class="tab-content">
                  <div class="breakdown-list">
                    <div class="breakdown-item">
                      <span class="label">Total Bookings</span>
                      <span class="value">{{ selectedVehicleAnalysis.totalBookings }}</span>
                    </div>
                    <div class="breakdown-item">
                      <span class="label">Average Booking Value</span>
                      <span class="value income">{{ formatPrice(selectedVehicleAnalysis.avgBookingValue) }}</span>
                    </div>
                    <div class="breakdown-item">
                      <span class="label">Total Days Rented</span>
                      <span class="value">{{ selectedVehicleAnalysis.totalDaysRented }} days</span>
                    </div>
                    <div class="breakdown-item">
                      <span class="label">Revenue per Day</span>
                      <span class="value income">{{ formatPrice(selectedVehicleAnalysis.revenuePerDay) }}</span>
                    </div>
                  </div>
                </div>

                <!-- Expense Details -->
                <div v-if="activeVehicleTab === 'expenses'" class="tab-content">
                  <div class="expense-breakdown">
                    <div
                        v-for="(amount, category) in selectedVehicleAnalysis.expensesByCategory"
                        :key="category"
                        class="expense-item"
                    >
                      <div class="expense-header">
                        <span class="category">{{ category }}</span>
                        <span class="amount">{{ formatPrice(amount) }}</span>
                      </div>
                      <div class="progress-bar">
                        <div
                            class="progress-fill"
                            :style="{ width: calculateExpensePercentage(category, selectedVehicleAnalysis) + '%' }"
                        ></div>
                      </div>
                    </div>
                  </div>
                </div>

                <!-- Breakdown Chart -->
                <div v-if="activeVehicleTab === 'breakdown'" class="tab-content">
                  <div class="chart-container">
                    <canvas :id="`vehicleChart-${selectedVehicleAnalysis.id}`"></canvas>
                  </div>
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
import {orderStatus} from "../utils/constants.ts";

export default {
  name: 'Summary',
  components: {
    HeaderComponent
  },
  data() {
    return {
      loggedUser: 'Admin User',
      currentDate: new Date().toLocaleString('en-US', {
        weekday: 'long',
        year: 'numeric',
        month: 'long',
        day: 'numeric'
      }),
      activeTab1: 'graph',
      activeTab2: 'graph',
      activeVehicleTab: 'income',
      transactions: [],
      accountSummery: [],
      vehicles: [],
      reservations: [],
      maintenanceRecords: [],
      expensesRecords: [],
      vehicleSearchQuery: '',
      selectedVehicleAnalysis: null,
      charts: {}
    };
  },

  computed: {
    currentMonthIncome() {
      const now = new Date();
      const monthStart = new Date(now.getFullYear(), now.getMonth(), 1);
      const monthEnd = new Date(now.getFullYear(), now.getMonth() + 1, 0);

      return this.transactions
          .filter(t => new Date(t.date) >= monthStart && new Date(t.date) <= monthEnd)
          .reduce((sum, t) => sum + t.amount, 0);
    },

    currentMonthExpenses() {
      const now = new Date();
      const monthStart = new Date(now.getFullYear(), now.getMonth(), 1);
      const monthEnd = new Date(now.getFullYear(), now.getMonth() + 1, 0);

      const expenses = this.expensesRecords
          .filter(t => new Date(t.date) >= monthStart && new Date(t.date) <= monthEnd)
          .reduce((sum, t) => sum + t.amount, 0);
      const maintenanceExpenses = this.maintenanceRecords
          .filter(t => new Date(t.serviceDate) >= monthStart && new Date(t.serviceDate) <= monthEnd)
          .reduce((sum, t) => sum + t.cost, 0);
      return maintenanceExpenses + expenses;
    },

    currentMonthProfit() {
      return this.currentMonthIncome - this.currentMonthExpenses;
    },

    profitMargin() {
      if (this.currentMonthIncome === 0) return 0;
      return Math.round((this.currentMonthProfit / this.currentMonthIncome) * 100);
    },

    expensePercentageOfIncome() {
      if (this.currentMonthIncome === 0) return 0;
      return Math.round((this.currentMonthExpenses / this.currentMonthIncome) * 100);
    },

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
        const expenses = expenses_credit + expenses_credit;
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

    vehiclesWithAnalysis() {
      return this.vehicles.map(vehicle => {
        const vehicleIncome = this.transactions
            .filter(r => r.vehicleId === vehicle.vehicleId)
            .reduce((sum, r) => sum + r.amount, 0);

        const vehicleExpenses = this.maintenanceRecords
            .filter(m => m.vehicleId === vehicle.vehicleId)
            .reduce((sum, m) => sum + m.cost, 0);

        const netProfit = vehicleIncome - vehicleExpenses;
        const profitMargin = vehicleIncome === 0 ? 0 : Math.round((netProfit / vehicleIncome) * 100);

        const totalBookings = this.reservations.filter(r => r.vehicleId === vehicle.vehicleId).length;
        const avgBookingValue = totalBookings === 0 ? 0 : vehicleIncome / totalBookings;

        const totalDaysRented = this.reservations
            .filter(r => r.vehicleId === vehicle.vehicleId && r.orderStatus === orderStatus.completed)
            .reduce((sum, r) => {
              const start = new Date(r.startTime);
              const end = new Date(r.endTime);
              return sum + Math.ceil((end - start) / (1000 * 60 * 60 * 24));
            }, 0);

        const revenuePerDay = totalDaysRented === 0 ? 0 : vehicleIncome / totalDaysRented;

        const expensesByCategory = {};
        this.maintenanceRecords
            .filter(m => m.vehicleId === vehicle.vehicleId)
            .forEach(m => {
              if (!expensesByCategory[m.maintenanceRecord]) {
                expensesByCategory[m.maintenanceRecord] = 0;
              }
              expensesByCategory[m.maintenanceRecord] += m.cost;
            });

        return {
          id: vehicle.id,
          registerNumber: vehicle.registerNumber,
          manufacturer: vehicle.manufacturer,
          modelName: vehicle.modelName,
          totalIncome: vehicleIncome,
          totalExpenses: vehicleExpenses,
          netProfit,
          profitMargin,
          totalBookings,
          avgBookingValue,
          totalDaysRented,
          revenuePerDay,
          expensesByCategory
        };
      });
    },

    filteredVehicles() {
      if (!this.vehicleSearchQuery) return this.vehiclesWithAnalysis;

      const query = this.vehicleSearchQuery.toLowerCase();
      return this.vehiclesWithAnalysis.filter(v =>
          v.registerNumber.toLowerCase().includes(query) ||
          v.manufacturer.toLowerCase().includes(query) ||
          v.modelName.toLowerCase().includes(query)
      );
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

    calculateExpensePercentage(category, vehicle) {
      const total = Object.values(vehicle.expensesByCategory).reduce((a, b) => a + b, 0);
      if (total === 0) return 0;
      return Math.round((vehicle.expensesByCategory[category] / total) * 100);
    },

    closeVehicleDetail() {
      this.selectedVehicleAnalysis = null;
    },

    initializeCurrentMonthChart() {
      const ctx = document.getElementById('currentMonthChart');
      if (!ctx) return;

      if (this.charts.currentMonth) {
        this.charts.currentMonth.destroy();
      }

      this.charts.currentMonth = new Chart(ctx, {
        type: 'doughnut',
        data: {
          labels: ['Income', 'Expenses'],
          datasets: [{
            data: [this.currentMonthIncome, this.currentMonthExpenses],
            backgroundColor: ['#22c55e', '#ef4444'],
            borderColor: 'var(--color-background-primary)',
            borderWidth: 3
          }]
        },
        options: {
          responsive: true,
          maintainAspectRatio: true,
          plugins: {
            legend: {
              position: 'bottom',
              labels: {
                usePointStyle: true,
                padding: 20,
                font: { size: 13, weight: '600' },
                color: 'var(--color-text-primary)'
              }
            }
          }
        }
      });
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
              borderRadius: 4,
              borderSkipped: false
            },
            {
              label: 'Expenses',
              data: expenseData,
              backgroundColor: '#ef4444',
              borderRadius: 4,
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

    initializeVehicleChart() {
      if (!this.selectedVehicleAnalysis) return;

      const canvasId = `vehicleChart-${this.selectedVehicleAnalysis.id}`;
      const ctx = document.getElementById(canvasId);
      if (!ctx) return;

      const chartKey = `vehicle-${this.selectedVehicleAnalysis.id}`;
      if (this.charts[chartKey]) {
        this.charts[chartKey].destroy();
      }

      const categories = Object.keys(this.selectedVehicleAnalysis.expensesByCategory);
      const amounts = Object.values(this.selectedVehicleAnalysis.expensesByCategory);

      this.charts[chartKey] = new Chart(ctx, {
        type: 'pie',
        data: {
          labels: categories,
          datasets: [{
            data: amounts,
            backgroundColor: [
              '#667eea',
              '#764ba2',
              '#f59e0b',
              '#ec4899',
              '#10b981',
              '#3b82f6',
              '#8b5cf6'
            ],
            borderColor: 'var(--color-background-primary)',
            borderWidth: 2
          }]
        },
        options: {
          responsive: true,
          maintainAspectRatio: true,
          plugins: {
            legend: {
              position: 'right',
              labels: {
                usePointStyle: true,
                padding: 15,
                font: { size: 12, weight: '600' },
                color: 'var(--color-text-primary)'
              }
            }
          }
        }
      });
    },

    handleLogout() {
      this.$emit('logout');
    },

    async getData() {
      try {
        // Fetch vehicles
        const vehiclesData = await dbService.getVehicles();
        this.vehicles = vehiclesData.map(item => ({
          id: item.id,
          vehicleId: item.vehicle_id,
          registerNumber: item.register_number,
          manufacturer: item.manufacturer,
          modelName: item.model_name
        }));

        // Fetch reservations
        const reservationData = await dbService.getAllOrders();
        this.reservations = reservationData.map(item => ({
          id: item.orderNumber,
          vehicleId: item.vehicleId,
          totalAmount: item.paidAmount,
          reservationStatus: item.orderStatus,
          startTime: item.releaseTime,
          endTime: item.handoverTime,
          createdTime: item.createdAt,
          orderStatus: item.orderStatus,
        }));

        // Fetch maintenance records
        const maintenanceData = await dbService.getMaintenanceRecords();
        this.maintenanceRecords = maintenanceData.map(item => ({
          id: item.id,
          vehicleId: item.vehicle_id,
          vehicleRegisterNumber: item.vehicle_register_number,
          maintenanceRecord: item.service_type + '-' + item.description,
          cost: item.cost,
          serviceDate: item.service_date,
        }));

        // Fetch expenses records
        const expenses =  await dbService.getExpenses();
        this.expensesRecords = expenses.map(item => ({
          id: item.id,
          amount: item.amount,
          date: item.date
        }))

        // Fetch transactions
        const transactionData = await dbService.getTransactions();
        this.transactions = transactionData.map(item => ({
          id: item.id,
          vehicleId: item.vehicle_id,
          amount: item.amount,
          paymentType: item.payment_type,
          date: item.created_at
        }))

        // Fetch Account Summery
        this.accountSummery = await dbService.getAccountSummery();
        console.log(this.accountSummery)

        this.$nextTick(() => {
          this.initializeCurrentMonthChart();
          this.initializeYearlyChart();
        });
      } catch (error) {
        console.log('Error fetching data:', error);
      }
    }
  },

  watch: {
    selectedVehicleAnalysis() {
      this.$nextTick(() => {
        this.initializeVehicleChart();
      });
    },
    activeVehicleTab() {
      this.$nextTick(() => {
        if (this.activeVehicleTab === 'breakdown') {
          this.initializeVehicleChart();
        }
      });
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
  max-width: 1600px;
  margin: 0 auto;
  width: 100%;
}

/* === PAGE HEADER === */
.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
}

.page-header h1 {
  margin: 0;
  font-size: 28px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.header-date {
  font-size: 13px;
  color: var(--color-text-secondary);
  font-weight: 600;
}

/* === ANALYSIS SECTION === */
.analysis-section {
  background: var(--color-background-primary);
  border-radius: var(--border-radius-lg);
  border: 1px solid var(--color-border-tertiary);
  padding: 1.5rem;
  margin-bottom: 2rem;
}

.section-title {
  font-size: 16px;
  font-weight: 700;
  color: var(--color-text-primary);
  margin-bottom: 1.5rem;
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

/* === STATS OVERVIEW === */
.stats-overview {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.stat-item {
  background: var(--color-background-secondary);
  padding: 1rem;
  border-radius: 8px;
  border: 1px solid var(--color-border-tertiary);
}

.stat-item label {
  display: block;
  font-size: 12px;
  color: var(--color-text-secondary);
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.3px;
  margin-bottom: 0.5rem;
}

.stat-value {
  margin: 0;
  font-size: 24px;
  font-weight: 700;
  color: var(--color-text-primary);
  font-family: monospace;
}

.stat-value.income {
  color: #22c55e;
}

.stat-value.expense {
  color: #ef4444;
}

.stat-value.negative {
  color: #ef4444;
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
  transition: all var(--transition-fast);
  margin-bottom: -2px;
}

.tab-button:hover {
  color: var(--color-text-primary);
}

.tab-button.active {
  color: var(--color-info);
  border-bottom-color: var(--color-info);
}

.tab-button.small {
  padding: 0.5rem 0.75rem;
  font-size: 12px;
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

#currentMonthChart,
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

.category-cell {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-weight: 600;
  color: var(--color-text-primary);
}

.category-cell.income {
  color: #22c55e;
}

.category-cell.expense {
  color: #ef4444;
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

.percentage.profit {
  color: var(--color-info);
}

.percentage.negative {
  color: #ef4444;
}

.chart-cell {
  text-align: center;
}

.mini-bar {
  height: 8px;
  border-radius: 4px;
  min-width: 40px;
  display: inline-block;
}

.mini-bar.income {
  background: linear-gradient(90deg, #22c55e, #16a34a);
}

.mini-bar.expense {
  background: linear-gradient(90deg, #ef4444, #dc2626);
}

.mini-bar.profit {
  background: linear-gradient(90deg, #3b82f6, #1e40af);
}

.total-row {
  background: var(--color-background-secondary);
  font-weight: 700;
  border-top: 2px solid var(--color-border-tertiary);
}

.month-cell {
  font-weight: 600;
  color: var(--color-text-primary);
}

/* === VEHICLE GRID === */
.filter-section {
  margin-bottom: 1.5rem;
}

.search-input {
  width: 100%;
  max-width: 300px;
  padding: 0.6rem 1rem;
  border: 1px solid var(--color-border-tertiary);
  border-radius: 4px;
  font-size: 13px;
  color: var(--color-text-primary);
  background: var(--color-background-secondary);
  transition: all var(--transition-fast);
}

.search-input:focus {
  outline: none;
  border-color: var(--color-info);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.vehicle-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 1.5rem;
}

.vehicle-card {
  background: var(--color-background-secondary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: 8px;
  padding: 1.5rem;
  cursor: pointer;
  transition: all var(--transition-normal);
}

.vehicle-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.1);
  border-color: var(--color-info);
}

.vehicle-header {
  margin-bottom: 1rem;
  padding-bottom: 1rem;
  border-bottom: 1px solid var(--color-border-tertiary);
}

.vehicle-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 700;
  color: var(--color-text-primary);
}

.vehicle-name {
  display: block;
  font-size: 12px;
  color: var(--color-text-secondary);
  margin-top: 0.25rem;
}

.vehicle-stats {
  display: grid;
  grid-template-columns: 1fr;
  gap: 1rem;
  margin-bottom: 1rem;
}

.vehicle-stat {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.vehicle-stat label {
  font-size: 12px;
  color: var(--color-text-secondary);
  font-weight: 600;
}

.vehicle-stat p {
  margin: 0;
  font-size: 14px;
  font-weight: 700;
  font-family: monospace;
}

.vehicle-stat .income {
  color: #22c55e;
}

.vehicle-stat .expense {
  color: #ef4444;
}

.vehicle-stat .negative {
  color: #ef4444;
}

.vehicle-footer {
  padding-top: 1rem;
  border-top: 1px solid var(--color-border-tertiary);
}

.profit-margin {
  display: inline-block;
  font-size: 12px;
  font-weight: 600;
  padding: 0.4rem 0.8rem;
  border-radius: 4px;
  background: rgba(239, 68, 68, 0.1);
  color: #dc2626;
}

.profit-margin.positive {
  background: rgba(34, 197, 94, 0.1);
  color: #16a34a;
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 3rem;
  color: var(--color-text-secondary);
}

.empty-state i {
  font-size: 64px;
  margin-bottom: 1rem;
  opacity: 0.3;
}

/* === MODAL === */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  background: var(--color-background-primary);
  border-radius: var(--border-radius-lg);
  width: 90%;
  max-width: 900px;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding: 1.5rem;
  border-bottom: 1px solid var(--color-border-tertiary);
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border-radius: var(--border-radius-lg) var(--border-radius-lg) 0 0;
}

.modal-header h2 {
  margin: 0;
  font-size: 20px;
}

.modal-header .vehicle-name {
  color: rgba(255, 255, 255, 0.9);
  margin-top: 0.25rem;
}

.btn-close {
  padding: 0.4rem;
  border: none;
  background: transparent;
  color: white;
  cursor: pointer;
  font-size: 20px;
  transition: color var(--transition-fast);
}

.btn-close:hover {
  color: rgba(255, 255, 255, 0.8);
}

.modal-body {
  padding: 2rem;
}

.vehicle-summary {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin-bottom: 2rem;
}

.summary-item {
  background: var(--color-background-secondary);
  padding: 1rem;
  border-radius: 8px;
  border: 1px solid var(--color-border-tertiary);
}

.summary-item label {
  display: block;
  font-size: 12px;
  color: var(--color-text-secondary);
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.3px;
  margin-bottom: 0.5rem;
}

.summary-item p {
  margin: 0;
  font-size: 18px;
  font-weight: 700;
  color: var(--color-text-primary);
  font-family: monospace;
}

.summary-item .income {
  color: #22c55e;
}

.summary-item .expense {
  color: #ef4444;
}

.breakdown-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.breakdown-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem;
  background: var(--color-background-secondary);
  border-radius: 8px;
  border: 1px solid var(--color-border-tertiary);
}

.breakdown-item .label {
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.breakdown-item .value {
  font-size: 14px;
  font-weight: 700;
  color: var(--color-text-primary);
  font-family: monospace;
}

.breakdown-item .value.income {
  color: #22c55e;
}

.expense-breakdown {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.expense-item {
  padding: 1rem;
  background: var(--color-background-secondary);
  border-radius: 8px;
}

.expense-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.75rem;
}

.expense-header .category {
  font-weight: 600;
  color: var(--color-text-primary);
}

.expense-header .amount {
  font-weight: 700;
  color: #ef4444;
  font-family: monospace;
}

.progress-bar {
  height: 8px;
  background: var(--color-border-tertiary);
  border-radius: 4px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #667eea, #764ba2);
  border-radius: 4px;
}

/* === RESPONSIVE === */
@media (max-width: 1024px) {
  .page-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.5rem;
  }

  .vehicle-grid {
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  }

  .modal-content {
    max-width: 95%;
  }
}

@media (max-width: 768px) {
  .summary-container {
    padding: 1rem;
  }

  .page-header h1 {
    font-size: 22px;
  }

  .stats-overview {
    grid-template-columns: repeat(2, 1fr);
  }

  .stat-value {
    font-size: 18px;
  }

  .tab-buttons {
    gap: 0.5rem;
  }

  .tab-button {
    padding: 0.5rem 0.75rem;
    font-size: 12px;
  }

  .vehicle-grid {
    grid-template-columns: 1fr;
  }

  .vehicle-summary {
    grid-template-columns: repeat(2, 1fr);
  }

  .analysis-table {
    font-size: 12px;
  }

  .analysis-table th,
  .analysis-table td {
    padding: 0.75rem;
  }

  .search-input {
    max-width: 100%;
  }
}

/* Scrollbar styling */
.summary-container::-webkit-scrollbar,
.table-wrapper::-webkit-scrollbar,
.modal-content::-webkit-scrollbar {
  width: 6px;
}

.summary-container::-webkit-scrollbar-track,
.table-wrapper::-webkit-scrollbar-track,
.modal-content::-webkit-scrollbar-track {
  background: transparent;
}

.summary-container::-webkit-scrollbar-thumb,
.table-wrapper::-webkit-scrollbar-thumb,
.modal-content::-webkit-scrollbar-thumb {
  background: var(--color-border-secondary);
  border-radius: 3px;
}

.summary-container::-webkit-scrollbar-thumb:hover,
.table-wrapper::-webkit-scrollbar-thumb:hover,
.modal-content::-webkit-scrollbar-thumb:hover {
  background: var(--color-border-primary);
}

/* Pending Settlements Section */
.pending-settlements-section {
  background: var(--color-background-primary);
  border-radius: var(--border-radius-lg);
  border: 1px solid var(--color-border-tertiary);
  padding: 1.5rem;
  margin-bottom: 2rem;
  margin-top: 2rem;
  animation: slideUp 0.4s ease;
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* Section Header */
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
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin: 0;
}

.section-title i {
  font-size: 18px;
  color: #f59e0b;
}

.section-subtitle {
  font-size: 13px;
  color: var(--color-text-secondary);
  margin: 0.5rem 0 0 0;
  font-weight: 500;
}

.settlement-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #f59e0b, #f97316);
  color: white;
  padding: 0.4rem 0.8rem;
  border-radius: 20px;
  font-size: 12px;
  font-weight: 700;
  white-space: nowrap;
  box-shadow: 0 2px 8px rgba(245, 158, 11, 0.2);
}

/* Settlements Container */
.settlements-container {
  display: flex;
  flex-direction: column;
}

.settlements-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 1.25rem;
}

/* Settlement Card */
.settlement-card {
  background: linear-gradient(135deg, var(--color-background-secondary) 0%, rgba(245, 158, 11, 0.02) 100%);
  border: 1px solid var(--color-border-tertiary);
  border-radius: 12px;
  padding: 1.25rem;
  transition: all var(--transition-normal);
  cursor: pointer;
  position: relative;
  overflow: hidden;
}

.settlement-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 3px;
  background: linear-gradient(90deg, #f59e0b, #f97316);
  transform: scaleX(0);
  transform-origin: left;
  transition: transform var(--transition-normal);
}

.settlement-card:hover {
  border-color: #f59e0b;
  transform: translateY(-3px);
  box-shadow: 0 8px 24px rgba(245, 158, 11, 0.15);
}

.settlement-card:hover::before {
  transform: scaleX(1);
}

/* Settlement Content */
.settlement-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.settlement-left {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.settlement-icon {
  width: 44px;
  height: 44px;
  border-radius: 10px;
  background: linear-gradient(135deg, rgba(245, 158, 11, 0.1), rgba(249, 115, 22, 0.1));
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
  color: #f59e0b;
}

.settlement-info {
  display: flex;
  flex-direction: column;
}

.settlement-label {
  font-size: 12px;
  color: var(--color-text-secondary);
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.3px;
}

.settlement-id {
  font-size: 14px;
  color: var(--color-text-primary);
  font-weight: 700;
}

.settlement-amount {
  font-size: 20px;
  font-weight: 700;
  color: #f59e0b;
  font-family: 'Courier New', monospace;
  text-align: right;
}

/* Settlement Footer */
.settlement-footer {
  padding-top: 1rem;
  border-top: 1px solid rgba(245, 158, 11, 0.1);
}

.status-badge {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  font-size: 11px;
  font-weight: 700;
  padding: 0.35rem 0.75rem;
  border-radius: 8px;
  text-transform: uppercase;
  letter-spacing: 0.3px;
}

.status-badge.pending {
  background: rgba(245, 158, 11, 0.15); color: #b45309;
  border: 1px solid rgba(245, 158, 11, 0.25);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.status-badge i {
  font-size: 10px;
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}

/* Empty State */
.empty-settlements {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 3rem 2rem;
  text-align: center;
}

.empty-icon {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  background: linear-gradient(135deg, rgba(34, 197, 94, 0.1), rgba(16, 185, 129, 0.1));
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 40px;
  color: #10b981;
  margin-bottom: 1rem;
}

.empty-title {
  font-size: 16px;
  font-weight: 700;
  color: var(--color-text-primary);
  margin: 0 0 0.5rem 0;
}

.empty-message {
  font-size: 13px;
  color: var(--color-text-secondary);
  margin: 0;
  max-width: 300px;
}

/* Responsive Design */
@media (max-width: 768px) {
  .pending-settlements-section {
    padding: 1rem;
    margin-bottom: 1.5rem;
    margin-top: 1.5rem;
  }

  .section-header {
    flex-direction: column;
    align-items: flex-start;
  }

  .settlement-badge {
    align-self: flex-start;
  }

  .settlements-grid {
    grid-template-columns: 1fr;
    gap: 1rem;
  }

  .settlement-card {
    padding: 1rem;
  }

  .settlement-content {
    flex-direction: column;
    align-items: flex-start;
  }

  .settlement-amount {
    width: 100%;
    text-align: left;
    margin-top: 0.75rem;
  }

  .settlement-footer {
    padding-top: 0.75rem;
  }
}

@media (max-width: 480px) {
  .section-title {
    font-size: 14px;
  }

  .settlement-card {
    border-radius: 8px;
  }

  .settlement-icon {
    width: 40px;
    height: 40px;
    font-size: 18px;
  }

  .settlement-amount {
    font-size: 18px;
  }

  .empty-icon {
    width: 60px;
    height: 60px;
    font-size: 32px;
  }
}
</style>
