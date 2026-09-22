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
            <h1> {{monthName}} Financial Summary</h1>
            <p class="header-subtitle">Your business overview at a glance</p>
          </div>
          <div class="header-nav">
            <router-link to="/summery-yearly" class="nav-link">
              <i class="fa fa-chart-bar"></i>
              Yearly Report
            </router-link>
            <router-link to="/summery-vehicle" class="nav-link">
              <i class="fa fa-car"></i>
              Vehicle Analytics
            </router-link>
          </div>
        </div>

        <!-- === KEY METRICS SECTION === -->
        <div class="metrics-section">
          <div class="metrics-grid">
            <!-- Net Profit - Hero Metric -->
            <div class="metric-hero">
              <div class="metric-label">Net Profit</div>
              <div class="metric-value" :class="{ 'negative': currentMonthProfit < 0 }">
                {{ formatPrice(currentMonthProfit) }}
              </div>
              <div class="metric-subtext">
                <span class="metric-trend" :class="{ 'negative': currentMonthProfit < 0 }">
                  <i :class="currentMonthProfit >= 0 ? 'fa fa-arrow-up' : 'fa fa-arrow-down'"></i>
                  {{ profitMargin }}% margin
                </span>
                <span class="metric-period">{{ monthName }}</span>
              </div>
            </div>

            <!-- Supporting Metrics -->
            <div class="metrics-support">
              <div class="metric-card">
                <div class="metric-card-label">Total Income</div>
                <div class="metric-card-value income">{{ formatPrice(currentMonthIncome) }}</div>
              </div>
              <div class="metric-card">
                <div class="metric-card-label">Total Expenses</div>
                <div class="metric-card-value expense">{{ formatPrice(currentMonthExpenses) }}</div>
              </div>
              <div class="metric-card">
                <div class="metric-card-label">Total Orders</div>
                <div class="metric-card-value">{{ totalOrders }}</div>
              </div>
            </div>
          </div>
        </div>

        <!-- === INCOME BREAKDOWN BY PAYMENT METHOD === -->
        <div class="breakdown-section">
          <div class="breakdown-header">
            <h2>Income Breakdown</h2>
            <p class="breakdown-subtitle">by payment method</p>
          </div>
          <div class="payment-methods-grid">
            <div class="payment-card income-card">
              <div class="payment-icon cash">
                <i class="fa fa-money"></i>
              </div>
              <div class="payment-info">
                <div class="payment-label">Cash</div>
                <div class="payment-amount">{{ formatPrice(incomeByMethod.cash) }}</div>
              </div>
              <div class="payment-bar">
                <div class="payment-bar-fill" :style="{ width: getPercentage(incomeByMethod.cash, currentMonthIncome) + '%' }"></div>
              </div>
            </div>

            <div class="payment-card income-card">
              <div class="payment-icon credit">
                <i class="fa fa-credit-card"></i>
              </div>
              <div class="payment-info">
                <div class="payment-label">Credit Card</div>
                <div class="payment-amount">{{ formatPrice(incomeByMethod.credit) }}</div>
              </div>
              <div class="payment-bar">
                <div class="payment-bar-fill" :style="{ width: getPercentage(incomeByMethod.credit, currentMonthIncome) + '%' }"></div>
              </div>
            </div>

            <div class="payment-card income-card">
              <div class="payment-icon bank">
                <i class="fa fa-bank"></i>
              </div>
              <div class="payment-info">
                <div class="payment-label">Bank Transfer</div>
                <div class="payment-amount">{{ formatPrice(incomeByMethod.bank) }}</div>
              </div>
              <div class="payment-bar">
                <div class="payment-bar-fill" :style="{ width: getPercentage(incomeByMethod.bank, currentMonthIncome) + '%' }"></div>
              </div>
            </div>
          </div>
        </div>

        <!-- === EXPENSE BREAKDOWN BY PAYMENT METHOD === -->
        <div class="breakdown-section">
          <div class="breakdown-header">
            <h2>Expense Breakdown</h2>
            <p class="breakdown-subtitle">by payment method</p>
          </div>
          <div class="payment-methods-grid">
            <div class="payment-card expense-card">
              <div class="payment-icon cash">
                <i class="fa fa-money"></i>
              </div>
              <div class="payment-info">
                <div class="payment-label">Cash</div>
                <div class="payment-amount">{{ formatPrice(expenseByMethod.cash) }}</div>
              </div>
              <div class="payment-bar">
                <div class="payment-bar-fill" :style="{ width: getPercentage(expenseByMethod.cash, currentMonthExpenses) + '%' }"></div>
              </div>
            </div>

            <div class="payment-card expense-card">
              <div class="payment-icon credit">
                <i class="fa fa-credit-card"></i>
              </div>
              <div class="payment-info">
                <div class="payment-label">Credit Card</div>
                <div class="payment-amount">{{ formatPrice(expenseByMethod.credit) }}</div>
              </div>
              <div class="payment-bar">
                <div class="payment-bar-fill" :style="{ width: getPercentage(expenseByMethod.credit, currentMonthExpenses) + '%' }"></div>
              </div>
              <div class="pending-badge" v-if="pendingCreditBalance > 0">
                {{ formatPrice(pendingCreditBalance) }} pending
              </div>
            </div>

            <div class="payment-card expense-card">
              <div class="payment-icon bank">
                <i class="fa fa-bank"></i>
              </div>
              <div class="payment-info">
                <div class="payment-label">Bank Transfer</div>
                <div class="payment-amount">{{ formatPrice(expenseByMethod.bank) }}</div>
              </div>
              <div class="payment-bar">
                <div class="payment-bar-fill" :style="{ width: getPercentage(expenseByMethod.bank, currentMonthExpenses) + '%' }"></div>
              </div>
            </div>
          </div>
        </div>

        <!-- === BANKED RECORDS SECTION === -->
        <div class="banked-records-section">
          <div class="banked-header">
            <div>
              <h2>
                <i class="fa fa-university"></i>
                Banked Records
              </h2>
              <p class="banked-subtitle">Daily bank deposit records</p>
            </div>
            <button class="view-records-btn" @click="showBankedModal = true">
              <i class="fa fa-list"></i>
              View All Records
            </button>
          </div>

          <div class="banked-records-container">
            <div
                v-if="dailyBankedRecords.length > 0"
                class="banked-cards-grid"
            >
              <div
                  v-for="(record, index) in displayedBankedRecords"
                  :key="index"
                  class="banked-card"
              >
                <div class="banked-card-header">
                  <div class="banked-date">{{ formatBankedDate(record.date) }}</div>
                  <div class="banked-amount">{{ formatPrice(record.amount) }}</div>
                </div>
                <div class="banked-card-footer">
                  <span class="banked-reference">Ref: {{ record.reference || 'N/A' }}</span>
                </div>
              </div>
            </div>

            <!-- Empty State -->
            <div v-else class="empty-banked">
              <div class="empty-icon">
                <i class="fa fa-inbox"></i>
              </div>
              <p class="empty-title">No Banked Records</p>
              <p class="empty-message">No bank deposits recorded yet</p>
            </div>

            <!-- View All Button -->
            <div v-if="dailyBankedRecords.length > 3" class="view-all-section banked-view-all">
              <button class="view-all-btn" @click="showBankedModal = true">
                View All {{ dailyBankedRecords.length }} Records
              </button>
            </div>
          </div>
        </div>

        <!-- === PENDING SETTLEMENTS SECTION === -->
        <div class="pending-settlements-section">
          <div class="section-header">
            <div>
              <h2>
                <i class="fa fa-hourglass-half"></i>
                Pending Settlements
              </h2>
              <p class="section-subtitle">Expenses awaiting payment settlement</p>
            </div>
            <div class="settlement-badge" v-if="pendingSettlement.length > 0">
              {{ pendingSettlement.length }} pending
            </div>
          </div>

          <!-- Settlements List -->
          <div class="settlements-container">
            <div
                v-if="pendingSettlement.length > 0"
                class="settlements-grid"
            >
              <div
                  v-for="(settlement, index) in displayedSettlements"
                  :key="index"
                  class="settlement-card"
              >
                <div class="settlement-icon">
                  <i class="fa fa-receipt"></i>
                </div>
                <div class="settlement-content">
                  <div class="settlement-header">
                    <span class="settlement-id">Settlement #{{ index + 1 }}</span>
                    <span class="settlement-amount">{{ formatPrice(settlement.amount) }}</span>
                  </div>
                  <button class="settle-btn" @click="handleSettleAction(index)">
                    <i class="fa fa-check"></i> Settle
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

            <!-- View All Button -->
            <div v-if="pendingSettlement.length > 3" class="view-all-section">
              <button class="view-all-btn" @click="showSettlementsModal = true">
                View All {{ pendingSettlement.length }} Settlements
              </button>
            </div>
          </div>
        </div>

      </div>
    </div>

    <!-- === SETTLEMENTS MODAL === -->
    <transition name="modal">
      <div v-if="showSettlementsModal" class="modal-overlay" @click.self="showSettlementsModal = false">
        <div class="modal-content">
          <div class="modal-header">
            <h2 class="modal-title">All Pending Settlements</h2>
            <button class="modal-close" @click="showSettlementsModal = false">
              <i class="fa fa-times"></i>
            </button>
          </div>

          <div class="modal-body">
            <div class="settlements-list">
              <div
                  v-for="(settlement, index) in pendingSettlement"
                  :key="index"
                  class="settlement-item"
              >
                <div class="settlement-item-left">
                  <div class="settlement-item-icon">
                    <i class="fa fa-receipt"></i>
                  </div>
                  <div class="settlement-item-info">
                    <div class="settlement-item-label">Settlement #{{ index + 1 }}</div>
                    <div class="settlement-item-date">{{ formatSettlementDate(settlement.date) }}</div>
                  </div>
                </div>
                <div class="settlement-item-right">
                  <div class="settlement-item-amount">{{ formatPrice(settlement.amount) }}</div>
                  <div class="settlement-item-actions">
                    <button
                        class="action-btn settle-btn"
                        @click="handleModalSettleAction(index)"
                        title="Mark as settled"
                    >
                      <i class="fa fa-check"></i>
                    </button>
                    <button
                        class="action-btn edit-btn"
                        @click="handleEditSettlement(index)"
                        title="Edit settlement"
                    >
                      <i class="fa fa-edit"></i>
                    </button>
                    <button
                        class="action-btn delete-btn"
                        @click="handleDeleteSettlement(index)"
                        title="Delete settlement"
                    >
                      <i class="fa fa-trash"></i>
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <div class="modal-footer">
            <button class="btn-secondary" @click="showSettlementsModal = false">
              Close
            </button>
          </div>
        </div>
      </div>
    </transition>

    <!-- === BANKED RECORDS MODAL === -->
    <transition name="modal">
      <div v-if="showBankedModal" class="modal-overlay" @click.self="showBankedModal = false">
        <div class="modal-content banked-modal">
          <div class="modal-header">
            <h2 class="modal-title">All Banked Records</h2>
            <button class="modal-close" @click="showBankedModal = false">
              <i class="fa fa-times"></i>
            </button>
          </div>

          <div class="modal-body">
            <div class="banked-records-list">
              <div
                  v-for="(record, index) in dailyBankedRecords"
                  :key="index"
                  class="banked-record-item"
              >
                <div class="banked-record-left">
                  <div class="banked-record-icon">
                    <i class="fa fa-university"></i>
                  </div>
                  <div class="banked-record-info">
                    <div class="banked-record-date">{{ formatBankedDate(record.date) }}</div>
                    <div class="banked-record-ref">{{ record.reference || 'No reference' }}</div>
                  </div>
                </div>
                <div class="banked-record-amount">{{ formatPrice(record.amount) }}</div>
              </div>
            </div>
          </div>

          <div class="modal-footer">
            <div class="banked-summary">
              <span class="summary-label">Total Banked:</span>
              <span class="summary-amount">{{ formatPrice(totalBankedAmount) }}</span>
            </div>
            <button class="btn-secondary" @click="showBankedModal = false">
              Close
            </button>
          </div>
        </div>
      </div>
    </transition>

  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue';
import HeaderComponent from '../component/Header.vue';
import { dbService } from '../services/db.ts';
import {paymentStatus, paymentTypes} from '../utils/constants.ts';
import { useSnackbar } from '../composables/useSnackbar'

// Refs
const loggedUser = ref('Admin User');
const transactions = ref([]);
const maintenanceRecords = ref([]);
const expensesRecords = ref([]);
const dailyBankedRecords = ref([]);
const showSettlementsModal = ref(false);
const showBankedModal = ref(false);
const { showSuccess, showError } = useSnackbar()

// Computed properties
const monthName = computed(() => {
  return new Date().toLocaleString('en-US', { month: 'long', year: 'numeric' });
});

const currentMonthIncome = computed(() => {
  const now = new Date();
  const monthStart = new Date(now.getFullYear(), now.getMonth(), 1);
  const monthEnd = new Date(now.getFullYear(), now.getMonth() + 1, 0);

  return transactions.value
      .filter(t => new Date(t.date) >= monthStart && new Date(t.date) <= monthEnd)
      .reduce((sum, t) => sum + t.amount, 0);
});

const currentMonthExpenses = computed(() => {
  const now = new Date();
  const monthStart = new Date(now.getFullYear(), now.getMonth(), 1);
  const monthEnd = new Date(now.getFullYear(), now.getMonth() + 1, 0);

  const expenses = expensesRecords.value
      .filter(t => new Date(t.date) >= monthStart && new Date(t.date) <= monthEnd)
      .reduce((sum, t) => sum + t.amount, 0);

  const maintenanceExpenses = maintenanceRecords.value
      .filter(t => new Date(t.serviceDate) >= monthStart && new Date(t.serviceDate) <= monthEnd)
      .reduce((sum, t) => sum + t.cost, 0);

  return maintenanceExpenses + expenses;
});

const currentMonthProfit = computed(() => {
  return currentMonthIncome.value - currentMonthExpenses.value;
});

const profitMargin = computed(() => {
  if (currentMonthIncome.value === 0) return 0;
  return Math.round((currentMonthProfit.value / currentMonthIncome.value) * 100);
});

const totalOrders = computed(() => {
  return transactions.value.length;
});

const incomeByMethod = computed(() => {
  const now = new Date();
  const monthStart = new Date(now.getFullYear(), now.getMonth(), 1);
  const monthEnd = new Date(now.getFullYear(), now.getMonth() + 1, 0);

  const current = transactions.value.filter(
      t => new Date(t.date) >= monthStart && new Date(t.date) <= monthEnd
  );

  return {
    cash: current.filter(t => t.paymentType === paymentTypes.cashPayment).reduce((sum, t) => sum + t.amount, 0),
    credit: current.filter(t => t.paymentType === paymentTypes.creditPayment).reduce((sum, t) => sum + t.amount, 0),
    bank: current.filter(t => t.paymentType === paymentTypes.bankTransfer).reduce((sum, t) => sum + t.amount, 0)
  };
});

const expenseByMethod = computed(() => {
  const now = new Date();
  const monthStart = new Date(now.getFullYear(), now.getMonth(), 1);
  const monthEnd = new Date(now.getFullYear(), now.getMonth() + 1, 0);

  const expenses = expensesRecords.value.filter(
      t => new Date(t.date) >= monthStart && new Date(t.date) <= monthEnd
  );

  const maintenance = maintenanceRecords.value.filter(
      t => new Date(t.serviceDate) >= monthStart && new Date(t.serviceDate) <= monthEnd
  );

  return {
    cash: expenses.filter(t => t.paymentType === paymentTypes.cashPayment).reduce((sum, t) => sum + t.amount, 0) +
        maintenance.filter(t => t.paymentType === paymentTypes.cashPayment).reduce((sum, t) => sum + t.cost, 0),
    credit: expenses.filter(t => t.paymentType === paymentTypes.creditPayment).reduce((sum, t) => sum + t.amount, 0) +
        maintenance.filter(t => t.paymentType === paymentTypes.creditPayment).reduce((sum, t) => sum + t.cost, 0),
    bank: expenses.filter(t => t.paymentType === paymentTypes.bankTransfer || paymentTypes.cardPayment).reduce((sum, t) => sum + t.amount, 0) +
        maintenance.filter(t => t.paymentType === paymentTypes.bankTransfer || paymentTypes.cardPayment).reduce((sum, t) => sum + t.cost, 0)
  };
});

const pendingCreditBalance = computed(() => {
  const pendingExpenses = expensesRecords.value
      .filter(item => item.status === paymentStatus.pending && item.paymentType === paymentTypes.creditPayment)
      .reduce((sum, item) => sum + item.amount, 0);

  const pendingMaintenance = maintenanceRecords.value
      .filter(item => item.status === paymentStatus.pending && item.paymentType === paymentTypes.creditPayment)
      .reduce((sum, item) => sum + item.cost, 0);

  return pendingExpenses + pendingMaintenance;
});

const pendingSettlement = computed(() => {
  const expPending = expensesRecords.value
      .filter(item => item.status === paymentStatus.pending)
      .map(item => ({
        id: item.id,
        amount: item.amount,
        date: item.date,
        type: 'expense'
      }));

  const maintenancePending = maintenanceRecords.value
      .filter(item => item.status === paymentStatus.pending)
      .map(item => ({
        id: item.id,
        amount: item.cost,
        date: item.serviceDate,
        type: 'maintenance'
      }));

  return expPending.concat(maintenancePending);
});

const displayedSettlements = computed(() => {
  return pendingSettlement.value.slice(0, 3);
});

const displayedBankedRecords = computed(() => {
  return dailyBankedRecords.value.slice(0, 3);
});

const totalBankedAmount = computed(() => {
  return dailyBankedRecords.value.reduce((sum, record) => sum + record.amount, 0);
});

// Methods
const formatPrice = (price) => {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'LKR',
    minimumFractionDigits: 2,
    maximumFractionDigits: 2
  }).format(price || 0);
};

const formatBankedDate = (date) => {
  if (!date) return 'N/A';
  return new Date(date).toLocaleDateString('en-US', {
    month: 'short',
    day: 'numeric',
    year: 'numeric'
  });
};

const formatSettlementDate = (date) => {
  if (!date) return 'N/A';
  return new Date(date).toLocaleDateString('en-US', {
    month: 'short',
    day: 'numeric',
    year: 'numeric'
  });
};

const getPercentage = (value, total) => {
  if (total === 0) return 0;
  return Math.round((value / total) * 100);
};

const handleLogout = () => {
  // Handle logout logic
};

const handleSettleAction = async (index) => {
  const settlement = displayedSettlements.value[index];
  if (settlement.type === 'expense') {
    await dbService.completeExpense({
      id: settlement.id,
      payment_status: paymentStatus.completed
    })
    showSuccess('Successfully completed expense');
  } else if (settlement.type === 'maintenance') {
    await dbService.completeMaintenance({
      id: settlement.id,
      payment_status: paymentStatus.completed
    })
    showSuccess('Successfully completed maintenance');
  }
};

const handleModalSettleAction = (index) => {
  const settlement = pendingSettlement.value[index];
  console.log('Modal settle action for:', settlement);
};

const handleEditSettlement = (index) => {
  const settlement = pendingSettlement.value[index];
  console.log('Edit settlement:', settlement);
};

const handleDeleteSettlement = (index) => {
  if (confirm('Are you sure you want to delete this settlement?')) {
    console.log('Delete settlement:', pendingSettlement.value[index]);
  }
};

const handleSettleAllAction = () => {
  if (confirm(`Settle all ${pendingSettlement.value.length} settlements?`)) {
    console.log('Settling all settlements');
    showSettlementsModal.value = false;
  }
};

const getData = async () => {
  try {
    const maintenanceData = await dbService.getMaintenanceRecords();
    maintenanceRecords.value = maintenanceData.map(item => ({
      id: item.id,
      vehicleId: item.vehicle_id,
      maintenanceRecord: item.service_type + '-' + item.description,
      cost: item.cost,
      serviceDate: item.service_date,
      paymentType: item.payment_type,
      status: item.payment_status,
    }));

    const expenses = await dbService.getExpenses();
    expensesRecords.value = expenses.map(item => ({
      id: item.id,
      amount: item.amount,
      date: item.date,
      paymentType: item.payment_type,
      status: item.payment_status,
    }));

    const transactionData = await dbService.getTransactions();
    transactions.value = transactionData.map(item => ({
      id: item.id,
      vehicleId: item.vehicle_id,
      amount: item.amount,
      paymentType: item.payment_type,
      date: item.created_at,
      status: item.status,
    }));

    // Fetch daily banked records
    const bankedData = await dbService.getBankRecords()
    dailyBankedRecords.value = bankedData.map(item => ({
      id: item.id,
      date: item.deposit_date,
      amount: item.amount,
      reference: item.note || null,
      bank: item.bank_name,
    })).sort((a, b) => new Date(b.date) - new Date(a.date));

  } catch (error) {
    console.log('Error fetching data:', error);
  }
};

// Lifecycle
onMounted(async () => {
  await getData();
});
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
  padding: 2rem;
  max-width: 1400px;
  margin: 0 auto;
  width: 100%;
}

/* === PAGE HEADER === */
.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2.5rem;
  gap: 2rem;
  flex-wrap: wrap;
  background: linear-gradient(135deg, var(--color-background-primary), var(--color-background-secondary));
  border: 1px solid var(--color-border-tertiary);
  border-radius: 12px;
  padding: 2rem;
  backdrop-filter: blur(10px);
}

.page-header > div:first-child {
  flex: 1;
  min-width: 200px;
}

.page-header h1 {
  margin: 0;
  font-size: 32px;
  font-weight: 600;
  letter-spacing: -0.5px;
  color: var(--color-text-primary);
  line-height: 1.2;
  background: linear-gradient(135deg, var(--color-text-primary), var(--color-text-secondary));
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.header-subtitle {
  margin: 0.75rem 0 0 0;
  font-size: 14px;
  color: var(--color-text-secondary);
  font-weight: 400;
  letter-spacing: 0.3px;
}

.header-nav {
  display: flex;
  gap: 1rem;
  flex-wrap: wrap;
  align-items: center;
}

.nav-link {
  display: flex;
  align-items: center;
  gap: 0.6rem;
  padding: 0.85rem 1.5rem;
  background: var(--color-background-primary);
  border: 1.5px solid var(--color-border-secondary);
  color: var(--color-text-primary);
  border-radius: 8px;
  text-decoration: none;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  white-space: nowrap;
  position: relative;
  overflow: hidden;
}

.nav-link::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
  transition: left 0.5s ease;
}

.nav-link:hover {
  border-color: var(--color-info);
  background: linear-gradient(135deg, var(--color-info-subtle), rgba(59, 130, 246, 0.08));
  color: var(--color-info);
  transform: translateY(-2px);
  box-shadow: 0 8px 16px rgba(59, 130, 246, 0.2);
}

.nav-link:hover::before {
  left: 100%;
}

.nav-link i {
  font-size: 14px;
  transition: transform 0.3s ease;
}

.nav-link:hover i {
  transform: scale(1.15);
}

/* === METRICS SECTION === */
.metrics-section {
  background: var(--color-background-primary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: 12px;
  padding: 2rem;
  margin-bottom: 2rem;
}

.metrics-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 2.5rem;
  align-items: center;
}

.metric-hero {
  display: flex;
  flex-direction: column;
  justify-content: center;
}

.metric-label {
  font-size: 13px;
  color: var(--color-text-secondary);
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-bottom: 0.75rem;
}

.metric-value {
  font-size: 52px;
  font-weight: 600;
  color: #10b981;
  font-family: 'SF Mono', Monaco, monospace;
  letter-spacing: -1px;
  margin-bottom: 1rem;
  line-height: 1;
}

.metric-value.negative {
  color: #ef4444;
}

.metric-subtext {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.metric-trend {
  display: inline-flex;
  align-items: center;
  gap: 0.4rem;
  font-size: 13px;
  font-weight: 500;
  color: #10b981;
}

.metric-trend.negative {
  color: #ef4444;
}

.metric-period {
  font-size: 13px;
  color: var(--color-text-secondary);
  font-weight: 400;
}

.metrics-support {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 1.5rem;
}

.metric-card {
  background: var(--color-background-secondary);
  padding: 1.5rem;
  border-radius: 8px;
  border: 1px solid var(--color-border-tertiary);
  text-align: center;
}

.metric-card-label {
  font-size: 12px;
  color: var(--color-text-secondary);
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.3px;
  margin-bottom: 0.75rem;
}

.metric-card-value {
  font-size: 24px;
  font-weight: 600;
  font-family: 'SF Mono', Monaco, monospace;
  color: var(--color-text-primary);
}

.metric-card-value.income {
  color: #10b981;
}

.metric-card-value.expense {
  color: #ef4444;
}

/* === BREAKDOWN SECTIONS === */
.breakdown-section {
  margin-bottom: 2rem;
}

.breakdown-header {
  margin-bottom: 1.5rem;
}

.breakdown-header h2 {
  margin: 0 0 0.5rem 0;
  font-size: 18px;
  font-weight: 500;
  color: var(--color-text-primary);
}

.breakdown-subtitle {
  margin: 0;
  font-size: 13px;
  color: var(--color-text-secondary);
  font-weight: 400;
}

.payment-methods-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 1.5rem;
}

.payment-card {
  background: var(--color-background-primary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: 10px;
  padding: 1.5rem;
  transition: all 0.2s ease;
  position: relative;
}

.payment-card:hover {
  border-color: var(--color-border-secondary);
}

.payment-icon {
  width: 48px;
  height: 48px;
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 22px;
  margin-bottom: 1rem;
  color: white;
}

.payment-icon.cash {
  background: linear-gradient(135deg, #3b82f6, #1e40af);
}

.payment-icon.credit {
  background: linear-gradient(135deg, #8b5cf6, #6d28d9);
}

.payment-icon.bank {
  background: linear-gradient(135deg, #06b6d4, #0891b2);
}

.payment-info {
  margin-bottom: 1rem;
}

.payment-label {
  font-size: 12px;
  color: var(--color-text-secondary);
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.3px;
  margin-bottom: 0.5rem;
}

.payment-amount {
  font-size: 24px;
  font-weight: 600;
  font-family: 'SF Mono', Monaco, monospace;
  color: var(--color-text-primary);
}

.payment-bar {
  height: 6px;
  background: var(--color-background-secondary);
  border-radius: 3px;
  overflow: hidden;
  margin-bottom: 1rem;
}

.payment-bar-fill {
  height: 100%;
  border-radius: 3px;
  background: var(--color-info);
  transition: width 0.3s ease;
}

.payment-card.income-card .payment-bar-fill {
  background: linear-gradient(90deg, #10b981, #059669);
}

.payment-card.expense-card .payment-bar-fill {
  background: linear-gradient(90deg, #ef4444, #dc2626);
}

.pending-badge {
  display: inline-block;
  font-size: 11px;
  color: #b45309;
  background: rgba(245, 158, 11, 0.1);
  padding: 0.3rem 0.75rem;
  border-radius: 4px;
  border: 1px solid rgba(245, 158, 11, 0.2);
  font-weight: 500;
}

/* === BANKED RECORDS SECTION === */
.banked-records-section {
  background: var(--color-background-primary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: 12px;
  padding: 2rem;
  margin-bottom: 2rem;
}

.banked-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 1.5rem;
  gap: 1.5rem;
  flex-wrap: wrap;
}

.banked-header h2 {
  margin: 0 0 0.5rem 0;
  font-size: 18px;
  font-weight: 500;
  color: var(--color-text-primary);
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.banked-header h2 i {
  color: #06b6d4;
}

.banked-subtitle {
  margin: 0;
  font-size: 13px;
  color: var(--color-text-secondary);
  font-weight: 400;
}

.view-records-btn {
  display: flex;
  align-items: center;
  gap: 0.6rem;
  padding: 0.75rem 1.25rem;
  background: var(--color-background-secondary);
  border: 1px solid var(--color-border-secondary);
  color: var(--color-text-primary);
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  white-space: nowrap;
}

.view-records-btn:hover {
  border-color: #06b6d4;
  background: rgba(6, 182, 212, 0.1);
}

.banked-records-container {
  display: flex;
  flex-direction: column;
}

.banked-cards-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 1.25rem;
  margin-bottom: 1.5rem;
}

.banked-card {
  background: var(--color-background-secondary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: 10px;
  padding: 1.25rem;
  transition: all 0.2s ease;
}

.banked-card:hover {
  border-color: #06b6d4;
  background: var(--color-background-primary);
}

.banked-card-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 1rem;
}

.banked-date {
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.banked-amount {
  font-size: 18px;
  font-weight: 700;
  color: #06b6d4;
  font-family: 'SF Mono', Monaco, monospace;
  white-space: nowrap;
}

.banked-card-footer {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.banked-reference {
  font-size: 11px;
  color: var(--color-text-secondary);
  font-family: 'SF Mono', Monaco, monospace;
  background: var(--color-background-tertiary);
  padding: 0.3rem 0.6rem;
  border-radius: 4px;
}

.empty-banked {
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
  background: linear-gradient(135deg, rgba(6, 182, 212, 0.1), rgba(8, 145, 178, 0.1));
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 40px;
  color: #06b6d4;
  margin-bottom: 1rem;
}

.empty-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 0 0 0.5rem 0;
}

.empty-message {
  font-size: 13px;
  color: var(--color-text-secondary);
  margin: 0;
  max-width: 300px;
}

.banked-view-all {
  border-top: 1px solid var(--color-border-tertiary);
  padding-top: 1.5rem;
}

.view-all-section {
  display: flex;
  justify-content: center;
  padding-top: 1.5rem;
  border-top: 1px solid var(--color-border-tertiary);
  margin-top: 1.5rem;
}

.view-all-btn {
  padding: 0.75rem 1.75rem;
  background: var(--color-background-secondary);
  border: 1px solid var(--color-border-secondary);
  color: var(--color-text-primary);
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
}

.view-all-btn:hover {
  border-color: #f59e0b;
  background: rgba(245, 158, 11, 0.08);
  color: #f59e0b;
  transform: translateY(-2px);
}

/* === PENDING SETTLEMENTS === */
.pending-settlements-section {
  background: var(--color-background-primary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: 12px;
  padding: 2rem;
  margin-bottom: 2rem;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
  gap: 1.5rem;
  flex-wrap: wrap;
}

.section-header h2 {
  margin: 0 0 0.5rem 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-primary);
  display: flex;
  align-items: center;
  gap: 0.75rem;
  line-height: 1.3;
}

.section-header h2 i {
  color: #f59e0b;
}

.section-subtitle {
  margin: 0;
  font-size: 13px;
  color: var(--color-text-secondary);
  font-weight: 400;
}

.settlement-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  background: linear-gradient(135deg, #f59e0b, #d97706);
  color: white;
  padding: 0.55rem 1.25rem;
  border-radius: 20px;
  font-size: 13px;
  font-weight: 600;
  white-space: nowrap;
  box-shadow: 0 2px 8px rgba(245, 158, 11, 0.3);
}

.settlements-container {
  display: flex;
  flex-direction: column;
}

.settlements-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 1.5rem;
  margin-bottom: 1.5rem;
}

.settlement-card {
  background: var(--color-background-secondary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: 10px;
  padding: 1.5rem;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1.5rem;
  transition: all 0.2s ease;
}

.settlement-card:hover {
  border-color: #f59e0b;
  background: var(--color-background-primary);
  box-shadow: 0 4px 12px rgba(245, 158, 11, 0.1);
}

.settlement-icon {
  width: 48px;
  height: 48px;
  border-radius: 10px;
  background: linear-gradient(135deg, rgba(245, 158, 11, 0.2), rgba(249, 115, 22, 0.2));
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 22px;
  color: #f59e0b;
  flex-shrink: 0;
}

.settlement-content {
  flex: 1;
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 1.5rem;
  min-width: 0;
}

.settlement-header {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  flex: 1;
  min-width: 0;
}

.settlement-id {
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-primary);
  line-height: 1.4;
}

.settlement-amount {
  font-size: 19px;
  font-weight: 700;
  color: #d97706;
  font-family: 'SF Mono', Monaco, monospace;
  white-space: nowrap;
  letter-spacing: -0.5px;
}

.settlement-card .settle-btn {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.625rem 1.25rem;
  background: linear-gradient(135deg, rgba(16, 185, 129, 0.15), rgba(5, 150, 105, 0.15));
  border: 1px solid rgba(16, 185, 129, 0.4);
  color: #059669;
  border-radius: 6px;
  font-size: 12px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
  white-space: nowrap;
  flex-shrink: 0;
}

.settlement-card .settle-btn:hover {
  background: linear-gradient(135deg, rgba(16, 185, 129, 0.25), rgba(5, 150, 105, 0.25));
  border-color: rgba(16, 185, 129, 0.6);
  transform: translateY(-2px);
}

.empty-settlements {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 3rem 2rem;
  text-align: center;
}

.empty-settlements .empty-icon {
  width: 80px;
  height: 80px;
  border-radius: 50%;
  background: linear-gradient(135deg, rgba(16, 185, 129, 0.1), rgba(5, 150, 105, 0.1));
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 40px;
  color: #10b981;
  margin-bottom: 1rem;
}

.empty-settlements .empty-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin: 0 0 0.5rem 0;
}

.empty-settlements .empty-message {
  font-size: 13px;
  color: var(--color-text-secondary);
  margin: 0;
  max-width: 300px;
}

/* === MODAL STYLES === */
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
  padding: 1rem;
}

.modal-content {
  background: var(--color-background-primary);
  border-radius: 12px;
  border: 1px solid var(--color-border-secondary);
  max-width: 700px;
  width: 100%;
  max-height: 80vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  animation: modalSlideIn 0.3s ease;
}

.modal-content.banked-modal {
  max-width: 800px;
}

@keyframes modalSlideIn {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  padding: 1.5rem;
  border-bottom: 1px solid var(--color-border-tertiary);
  gap: 1rem;
}

.modal-title {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.modal-close {
  background: transparent;
  border: none;
  font-size: 24px;
  color: var(--color-text-secondary);
  cursor: pointer;
  transition: all 0.2s ease;
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 6px;
  flex-shrink: 0;
}

.modal-close:hover {
  color: var(--color-text-primary);
  background: var(--color-background-secondary);
}

.modal-body {
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem;
}

.settlements-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.settlement-item {
  background: var(--color-background-secondary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: 8px;
  padding: 1.25rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1.5rem;
  transition: all 0.2s ease;
}

.settlement-item:hover {
  border-color: var(--color-border-secondary);
}

.settlement-item-left {
  display: flex;
  align-items: center;
  gap: 1rem;
  flex: 1;
  min-width: 0;
}

.settlement-item-icon {
  width: 40px;
  height: 40px;
  border-radius: 8px;
  background: linear-gradient(135deg, rgba(245, 158, 11, 0.15), rgba(249, 115, 22, 0.15));
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 18px;
  color: #f59e0b;
  flex-shrink: 0;
}

.settlement-item-info {
  flex: 1;
  min-width: 0;
}

.settlement-item-label {
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: 0.25rem;
}

.settlement-item-date {
  font-size: 12px;
  color: var(--color-text-secondary);
}

.settlement-item-right {
  display: flex;
  align-items: center;
  gap: 1.5rem;
  flex-wrap: wrap;
  justify-content: flex-end;
}

.settlement-item-amount {
  font-size: 16px;
  font-weight: 700;
  color: #f59e0b;
  font-family: 'SF Mono', Monaco, monospace;
  white-space: nowrap;
}

.settlement-item-actions {
  display: flex;
  gap: 0.5rem;
}

.action-btn {
  width: 38px;
  height: 38px;
  border: 1px solid var(--color-border-tertiary);
  background: var(--color-background-secondary);
  border-radius: 6px;
  color: var(--color-text-secondary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 15px;
  transition: all 0.2s ease;
  flex-shrink: 0;
}

.action-btn:hover {
  background: var(--color-background-primary);
  border-color: var(--color-border-secondary);
  transform: translateY(-2px);
}

.action-btn.settle-btn {
  border-color: rgba(16, 185, 129, 0.4);
  color: #059669;
}

.action-btn.settle-btn:hover {
  background: rgba(16, 185, 129, 0.1);
  border-color: rgba(16, 185, 129, 0.6);
}

.action-btn.edit-btn {
  border-color: rgba(59, 130, 246, 0.4);
  color: #1e40af;
}

.action-btn.edit-btn:hover {
  background: rgba(59, 130, 246, 0.1);
  border-color: rgba(59, 130, 246, 0.6);
}

.action-btn.delete-btn {
  border-color: rgba(239, 68, 68, 0.4);
  color: #dc2626;
}

.action-btn.delete-btn:hover {
  background: rgba(239, 68, 68, 0.1);
  border-color: rgba(239, 68, 68, 0.6);
}

/* === BANKED RECORDS LIST === */
.banked-records-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.banked-record-item {
  background: var(--color-background-secondary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: 8px;
  padding: 1.25rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1.5rem;
  transition: all 0.2s ease;
}

.banked-record-item:hover {
  border-color: #06b6d4;
  background: var(--color-background-primary);
}

.banked-record-left {
  display: flex;
  align-items: center;
  gap: 1rem;
  flex: 1;
  min-width: 0;
}

.banked-record-icon {
  width: 40px;
  height: 40px;
  border-radius: 8px;
  background: linear-gradient(135deg, rgba(6, 182, 212, 0.15), rgba(8, 145, 178, 0.15));
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 18px;
  color: #06b6d4;
  flex-shrink: 0;
}

.banked-record-info {
  flex: 1;
  min-width: 0;
}

.banked-record-date {
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: 0.25rem;
}

.banked-record-ref {
  font-size: 12px;
  color: var(--color-text-secondary);
  font-family: 'SF Mono', Monaco, monospace;
}

.banked-record-amount {
  font-size: 16px;
  font-weight: 700;
  color: #06b6d4;
  font-family: 'SF Mono', Monaco, monospace;
  white-space: nowrap;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  align-items: center;
  gap: 1rem;
  padding: 1.5rem;
  border-top: 1px solid var(--color-border-tertiary);
  flex-wrap: wrap;
}

.modal-footer .banked-summary {
  margin-right: auto;
}

.banked-summary {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.summary-label {
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.summary-amount {
  font-size: 16px;
  font-weight: 700;
  color: #06b6d4;
  font-family: 'SF Mono', Monaco, monospace;
}

.btn-primary,
.btn-secondary {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
  border: none;
}

.btn-primary {
  background: var(--color-info);
  color: white;
}

.btn-primary:hover {
  background: var(--color-info-dark);
  transform: translateY(-2px);
}

.btn-secondary {
  background: var(--color-background-secondary);
  color: var(--color-text-primary);
  border: 1px solid var(--color-border-tertiary);
}

.btn-secondary:hover {
  border-color: var(--color-border-secondary);
  background: var(--color-background-tertiary);
}

/* === MODAL TRANSITIONS === */
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-from .modal-content,
.modal-leave-to .modal-content {
  transform: translateY(20px);
}

/* === RESPONSIVE === */
@media (max-width: 1024px) {
  .summary-container {
    padding: 1.5rem;
  }

  .page-header {
    padding: 1.5rem;
    flex-direction: column;
    align-items: stretch;
  }

  .page-header > div:first-child {
    flex: 1;
  }

  .page-header h1 {
    font-size: 28px;
  }

  .header-nav {
    width: 100%;
    flex-direction: row;
    justify-content: flex-start;
  }

  .nav-link {
    flex: 1;
    justify-content: center;
  }

  .metrics-grid {
    grid-template-columns: 1fr;
  }

  .metrics-support {
    grid-template-columns: 1fr;
  }

  .banked-header {
    flex-direction: column;
  }

  .view-records-btn {
    width: 100%;
    justify-content: center;
  }
}

@media (max-width: 768px) {
  .summary-container {
    padding: 1rem;
  }

  .page-header {
    padding: 1.25rem;
    margin-bottom: 1.75rem;
  }

  .page-header h1 {
    font-size: 24px;
  }

  .header-subtitle {
    font-size: 13px;
    margin-top: 0.5rem;
  }

  .header-nav {
    width: 100%;
    gap: 0.75rem;
  }

  .nav-link {
    padding: 0.75rem 1rem;
    font-size: 12px;
    flex: 1;
    min-width: 0;
  }

  .nav-link i {
    font-size: 13px;
  }

  .metric-value {
    font-size: 36px;
  }

  .metric-card-value {
    font-size: 20px;
  }

  .payment-methods-grid {
    grid-template-columns: 1fr;
  }

  .banked-cards-grid {
    grid-template-columns: 1fr;
  }

  .settlements-grid {
    grid-template-columns: 1fr;
  }

  .settlement-card {
    flex-direction: column;
    text-align: center;
  }

  .settlement-content {
    flex-direction: column;
    width: 100%;
  }

  .modal-content {
    max-height: 90vh;
  }
}

/* Scrollbar styling */
.summary-container::-webkit-scrollbar,
.modal-body::-webkit-scrollbar {
  width: 6px;
}

.summary-container::-webkit-scrollbar-track,
.modal-body::-webkit-scrollbar-track {
  background: transparent;
}

.summary-container::-webkit-scrollbar-thumb,
.modal-body::-webkit-scrollbar-thumb {
  background: var(--color-border-secondary);
  border-radius: 3px;
}

.summary-container::-webkit-scrollbar-thumb:hover,
.modal-body::-webkit-scrollbar-thumb:hover {
  background: var(--color-border-primary);
}
</style>
