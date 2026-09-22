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
            <h1>Vehicle Analytics</h1>
            <p class="header-subtitle">Income & Expenses by Vehicle</p>
          </div>
          <div class="header-nav">
            <router-link to="/summery-overview" class="nav-link">
              <i class="fa fa-calendar"></i>
              <span>Current Month</span>
            </router-link>
            <router-link to="/summery-yearly" class="nav-link">
              <i class="fa fa-chart-bar"></i>
              <span>Yearly Report</span>
            </router-link>
          </div>
        </div>

        <!-- === QUICK STATS === -->
        <div class="quick-stats">
          <div class="quick-stat">
            <div class="stat-icon">
              <i class="fa fa-car"></i>
            </div>
            <div class="stat-info">
              <div class="stat-label">Total Vehicles</div>
              <div class="stat-value">{{ vehicles.length }}</div>
            </div>
          </div>
          <div class="quick-stat">
            <div class="stat-icon">
              <i class="fa fa-dollar"></i>
            </div>
            <div class="stat-info">
              <div class="stat-label">Total Income</div>
              <div class="stat-value income">{{ formatPrice(totalIncome) }}</div>
            </div>
          </div>
          <div class="quick-stat">
            <div class="stat-icon">
              <i class="fa fa-arrow-up"></i>
            </div>
            <div class="stat-info">
              <div class="stat-label">Total Expenses</div>
              <div class="stat-value expense">{{ formatPrice(totalExpenses) }}</div>
            </div>
          </div>
          <div class="quick-stat">
            <div class="stat-icon">
              <i class="fa fa-line-chart"></i>
            </div>
            <div class="stat-info">
              <div class="stat-label">Total Profit</div>
              <div class="stat-value" :class="{ 'negative': totalProfit < 0 }">
                {{ formatPrice(totalProfit) }}
              </div>
            </div>
          </div>
        </div>

        <!-- === FILTER & SEARCH === -->
        <div class="filter-section">
          <div class="filter-header">
            <h3>Select Vehicle</h3>
            <span class="filter-count">{{ filteredVehicles.length }} vehicles</span>
          </div>
          <select
              id="vehicle"
              v-model="vehicleSearchQuery"
              class="filter-select"
          >
            <option value="">All Vehicles</option>
            <option v-for="vehicle in vehicles" :key="vehicle.vehicleId" :value="vehicle.vehicleId">
              {{ vehicle.registerNumber }} - {{ vehicle.manufacturer }} {{ vehicle.modelName }}
            </option>
          </select>
        </div>

        <!-- === VEHICLE GRID === -->
        <div class="vehicles-section">
          <div v-if="filteredVehicles.length > 0" class="vehicle-grid">
            <div
                v-for="vehicle in filteredVehicles"
                :key="vehicle.id"
                class="vehicle-card"
                @click="selectedVehicleAnalysis = vehicle"
            >
              <div class="vehicle-header">
                <h3>{{ vehicle.registerNumber }}</h3>
                <span class="vehicle-badge">{{ vehicle.manufacturer }}</span>
              </div>
              <div class="vehicle-model">{{ vehicle.modelName }}</div>

              <div class="vehicle-stats">
                <div class="vehicle-stat">
                  <label>Income</label>
                  <p class="income">{{ formatPrice(vehicle.totalIncome) }}</p>
                </div>
                <div class="vehicle-stat">
                  <label>Expenses</label>
                  <p class="expense">{{ formatPrice(vehicle.totalExpenses) }}</p>
                </div>
<!--                <div class="vehicle-stat">-->
<!--                  <label>Profit</label>-->
<!--                  <p :class="{ 'negative': vehicle.netProfit < 0 }">-->
<!--                    {{ formatPrice(vehicle.netProfit) }}-->
<!--                  </p>-->
<!--                </div>-->
              </div>

              <div class="vehicle-footer">
                <span class="profit-margin" :class="{ 'positive': vehicle.profitMargin >= 0 }">
                  {{ vehicle.profitMargin }}% margin
                </span>
                <button class="view-btn">
                  <i class="fa fa-arrow-right"></i>
                </button>
              </div>
            </div>
          </div>

          <!-- Empty State -->
          <div v-else class="empty-state">
            <div class="empty-icon">
              <i class="fa fa-car"></i>
            </div>
            <p class="empty-title">No vehicles found</p>
            <p class="empty-message">Try adjusting your filters</p>
          </div>
        </div>

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
                  class="tab-button"
                  :class="{ active: activeVehicleTab === 'income' }"
                  @click="activeVehicleTab = 'income'"
              >
                <i class="fa fa-arrow-down"></i>
                Income Details
              </button>
              <button
                  class="tab-button"
                  :class="{ active: activeVehicleTab === 'expenses' }"
                  @click="activeVehicleTab = 'expenses'"
              >
                <i class="fa fa-arrow-up"></i>
                Expense Details
              </button>
              <button
                  class="tab-button"
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
</template>

<script>
import HeaderComponent from '../component/Header.vue';
import { dbService } from '../services/db.ts';
import Chart from 'chart.js/auto';
import {orderStatus} from "../utils/constants.ts";

export default {
  name: 'SummaryVehicles',
  components: {
    HeaderComponent
  },
  data() {
    return {
      loggedUser: 'Admin User',
      vehicles: [],
      transactions: [],
      reservations: [],
      maintenanceRecords: [],
      vehicleSearchQuery: '',
      selectedVehicleAnalysis: null,
      activeVehicleTab: 'income',
      charts: {}
    };
  },

  computed: {
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
          vehicleId: vehicle.vehicleId,
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

      return this.vehiclesWithAnalysis.filter(v => v.vehicleId === this.vehicleSearchQuery);
    },

    totalIncome() {
      return this.vehiclesWithAnalysis.reduce((sum, v) => sum + v.totalIncome, 0);
    },

    totalExpenses() {
      return this.vehiclesWithAnalysis.reduce((sum, v) => sum + v.totalExpenses, 0);
    },

    totalProfit() {
      return this.totalIncome - this.totalExpenses;
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

        // Fetch transactions
        const transactionData = await dbService.getTransactions();
        this.transactions = transactionData.map(item => ({
          id: item.id,
          vehicleId: item.vehicle_id,
          amount: item.amount,
          paymentType: item.payment_type,
          date: item.created_at
        }))
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

/* === QUICK STATS === */
.quick-stats {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1.5rem;
  margin-bottom: 2rem;
}

.quick-stat {
  background: var(--color-background-primary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-lg);
  padding: 1.5rem;
  display: flex;
  align-items: flex-start;
  gap: 1rem;
  transition: all 0.2s ease;
}

.quick-stat:hover {
  border-color: var(--color-border-secondary);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.05);
}

.stat-icon {
  width: 48px;
  height: 48px;
  border-radius: 10px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
  color: white;
  flex-shrink: 0;
}

.stat-info {
  flex: 1;
}

.stat-label {
  font-size: 12px;
  color: var(--color-text-secondary);
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.3px;
  margin-bottom: 0.4rem;
}

.stat-value {
  margin: 0;
  font-size: 20px;
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

/* === FILTER SECTION === */
.filter-section {
  background: var(--color-background-primary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-lg);
  padding: 1.5rem;
  margin-bottom: 2rem;
}

.filter-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.filter-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.filter-count {
  font-size: 12px;
  color: var(--color-text-secondary);
  font-weight: 600;
}

.filter-select {
  width: 100%;
  padding: 0.75rem 1rem;
  border: 1px solid var(--color-border-tertiary);
  border-radius: 8px;
  background: var(--color-background-secondary);
  color: var(--color-text-primary);
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
}

.filter-select:focus {
  outline: none;
  border-color: #667eea;
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}

/* === VEHICLE GRID === */
.vehicles-section {
  margin-bottom: 2rem;
}

.vehicle-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 1.5rem;
}

.vehicle-card {
  background: var(--color-background-primary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-lg);
  padding: 1.5rem;
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  flex-direction: column;
}

.vehicle-card:hover {
  transform: translateY(-6px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.1);
  border-color: #667eea;
}

.vehicle-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 0.5rem;
  gap: 1rem;
}

.vehicle-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 700;
  color: var(--color-text-primary);
}

.vehicle-badge {
  display: inline-block;
  font-size: 11px;
  font-weight: 600;
  padding: 0.3rem 0.7rem;
  border-radius: 4px;
  background: rgba(102, 126, 234, 0.1);
  color: #667eea;
  text-transform: uppercase;
  letter-spacing: 0.3px;
  white-space: nowrap;
}

.vehicle-model {
  font-size: 13px;
  color: var(--color-text-secondary);
  margin-bottom: 1rem;
  font-weight: 500;
}

.vehicle-stats {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
  gap: 1rem;
  margin-bottom: 1rem;
  padding: 1rem;
  background: var(--color-background-secondary);
  border-radius: 8px;
}

.vehicle-stat {
  text-align: center;
}

.vehicle-stat label {
  display: block;
  font-size: 11px;
  color: var(--color-text-secondary);
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.3px;
  margin-bottom: 0.5rem;
}

.vehicle-stat p {
  margin: 0;
  font-size: 14px;
  font-weight: 700;
  color: var(--color-text-primary);
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
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-top: 1rem;
  border-top: 1px solid var(--color-border-tertiary);
  margin-top: auto;
}

.profit-margin {
  display: inline-block;
  font-size: 12px;
  font-weight: 700;
  padding: 0.4rem 0.8rem;
  border-radius: 4px;
  background: rgba(239, 68, 68, 0.1);
  color: #dc2626;
}

.profit-margin.positive {
  background: rgba(34, 197, 94, 0.1);
  color: #16a34a;
}

.view-btn {
  width: 32px;
  height: 32px;
  border-radius: 6px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
  transition: all 0.3s ease;
}

.view-btn:hover {
  transform: scale(1.1);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
}

/* === EMPTY STATE === */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 4rem 2rem;
  background: var(--color-background-primary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-lg);
  color: var(--color-text-secondary);
}

.empty-icon {
  font-size: 80px;
  margin-bottom: 1rem;
  opacity: 0.2;
}

.empty-title {
  font-size: 18px;
  font-weight: 700;
  color: var(--color-text-primary);
  margin: 0 0 0.5rem 0;
}

.empty-message {
  font-size: 13px;
  color: var(--color-text-secondary);
  margin: 0;
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
  font-size: 13px;
}

.btn-close {
  padding: 0.4rem;
  border: none;
  background: transparent;
  color: white;
  cursor: pointer;
  font-size: 20px;
  transition: color 0.3s ease;
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
  font-size: 13px;
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

.chart-container {
  background: var(--color-background-secondary);
  padding: 1.5rem;
  border-radius: 8px;
  border: 1px solid var(--color-border-tertiary);
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

  .vehicle-grid {
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  }

  .quick-stats {
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

  .vehicle-grid {
    grid-template-columns: 1fr;
  }

  .quick-stats {
    grid-template-columns: 1fr;
    gap: 1rem;
  }

  .vehicle-summary {
    grid-template-columns: repeat(2, 1fr);
  }

  .modal-content {
    max-width: 95%;
  }
}

/* Scrollbar styling */
.summary-container::-webkit-scrollbar,
.modal-content::-webkit-scrollbar {
  width: 6px;
}

.summary-container::-webkit-scrollbar-track,
.modal-content::-webkit-scrollbar-track {
  background: transparent;
}

.summary-container::-webkit-scrollbar-thumb,
.modal-content::-webkit-scrollbar-thumb {
  background: var(--color-border-secondary);
  border-radius: 3px;
}

.summary-container::-webkit-scrollbar-thumb:hover,
.modal-content::-webkit-scrollbar-thumb:hover {
  background: var(--color-border-primary);
}
</style>
