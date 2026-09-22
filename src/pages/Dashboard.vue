<template>
  <div class="dashboard">
    <!-- Header Component -->
    <header-component
        :showBackButton="false"
        @logout="handleLogout"
    />

    <!-- Dashboard Content -->
    <div class="dashboard-content">
      <div class="dashboard-container">

        <!-- First Row: Make Order Button -->
        <div class="action-row">
          <button class="make-order-button" @click="navigateToOrders">
            <i class="fa fa-plus" aria-hidden="true"></i>
            <span>Make Order</span>
          </button>
          <button class="make-order-button" @click="navigateToCalenders">
            <i class="fa fa-calendar" aria-hidden="true"></i>
            <span>Check Availability</span>
          </button>
        </div>

        <!-- Dashboard Stats Grid -->
        <div class="stats-grid">
          <div class="stat-card">
            <div class="stat-icon today">
              <i class="fa fa-calendar"></i>
            </div>
            <div class="stat-content">
              <p class="stat-label">Today's Orders</p>
              <h3 class="stat-value">{{ todayOrders }}</h3>
            </div>
          </div>

          <div class="stat-card">
            <div class="stat-icon revenue">
              <i class="fa fa-dollar"></i>
            </div>
            <div class="stat-content">
              <p class="stat-label">Today's Revenue</p>
              <h3 class="stat-value">LKR {{ todayRevenue }}</h3>
            </div>
          </div>

          <div class="stat-card">
            <div class="stat-icon products">
              <i class="fa fa-list"></i>
            </div>
            <div class="stat-content">
              <p class="stat-label">Total Vehicles</p>
              <h3 class="stat-value">{{ totalProducts }}</h3>
            </div>
          </div>

          <div class="stat-card">
            <div class="stat-icon pending">
              <i class="fa fa-clock-o"></i>
            </div>
            <div class="stat-content">
              <p class="stat-label">Pending Orders</p>
              <h3 class="stat-value">{{ pendingOrders }}</h3>
            </div>
          </div>
        </div>

        <!-- Quick Links -->
        <div class="quick-links">
          <h2 class="section-title">Quick access</h2>
          <div class="links-grid">
            <router-link to="/view-order" class="quick-link">
              <span class="action-badge">Action Required</span>
              <i class="fa fa-check"></i>
              <span>{{ t('dashboard.quickLinks.viewOrder')}}</span>
            </router-link>
            <router-link to="/products" class="quick-link">
              <i class="fa fa-list"></i>
              <span>{{ t('dashboard.quickLinks.manageVehicle')}}</span>
            </router-link>
            <router-link to="/maintenance" class="quick-link">
              <span v-if="pendingMaintenance > 0 " class="action-badge"> {{ pendingMaintenance }} Action Required</span>
              <i class="fa fa-car"></i>
              <span>{{ t('dashboard.quickLinks.vehicleMaintenance')}}</span>
            </router-link>
            <router-link to="/expenses" class="quick-link">
              <span v-if="pendingExpenses > 0 " class="action-badge"> {{ pendingExpenses }} Action Required</span>
              <i class="fa fa-money"></i>
              <span>{{ t('dashboard.quickLinks.expenses')}}</span>
            </router-link>
            <router-link to="/gallery" class="quick-link">
              <i class="fa fa-image"></i>
              <span>{{ t('dashboard.quickLinks.quickOrder')}}</span>
            </router-link>
          </div>
          <div class="links-grid">
            <router-link to="/incident-handling" class="quick-link">
              <i class="fa fa-exclamation-circle"></i>
              <span>{{ t('dashboard.quickLinks.incident')}}</span>
            </router-link>
            <router-link to="/account" class="quick-link">
              <i class="fa fa-bank"></i>
              <span>{{ t('dashboard.quickLinks.account')}}</span>
            </router-link>
            <router-link v-if="isAuthenticated" to="/summery-overview" class="quick-link">
              <i class="fa fa-bars"></i>
              <span>{{ t('dashboard.quickLinks.currentMonthProgress')}}</span>
            </router-link>
            <div v-else class="quick-link disabled-link">
              <i class="fa fa-lock"></i>
              <span>{{ t('dashboard.quickLinks.currentMonthProgress')}}</span>
            </div>
<!--            <router-link v-if="isAuthenticated" to="/summery-yearly" class="quick-link">-->
<!--              <i class="fa fa-line-chart"></i>-->
<!--              <span>Summery Yearly</span>-->
<!--            </router-link>-->
            <router-link v-if="isAuthenticated" to="/summery-vehicle" class="quick-link">
              <i class="fa fa-line-chart"></i>
              <span>{{ t('dashboard.quickLinks.vehicleOverview')}}</span>
            </router-link>
            <router-link v-if="isAuthenticated" to="/settings" class="quick-link" >
              <i class="fa fa-gear"></i>
              <span>Settings</span>
            </router-link>
            <div v-else class="quick-link disabled-link">
              <i class="fa fa-lock"></i>
              <span>Settings</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, defineEmits } from 'vue';
import { useRouter } from 'vue-router';
import HeaderComponent from '../component/Header.vue';
import { dbService } from "../services/db.ts";
import { orderStatus, paymentStatus, roleTypes } from "../utils/constants.ts";
import { useAuthStore } from "../stores/auth.ts";
import { useI18n } from "vue-i18n";

defineProps({
  name: {
    type: String,
    default: 'Dashboard'
  }
});

const emit = defineEmits(['logout']);
const {t, locale} = useI18n();

// ============ Setup ============
const router = useRouter();
const authStore = useAuthStore();

// ============ Reactive State ============
const loggedUser = ref(authStore.username);
const roleName = ref(authStore.role);
const todayOrders = ref(24);
const todayRevenue = ref(1280.50);
const totalProducts = ref(156);
const pendingOrders = ref(5);
const pendingMaintenance = ref(0);
const pendingExpenses = ref(0);

// ============ Computed ============
const isAuthenticated = computed(() => {
  return roleName.value === roleTypes.admin;
});

// ============ Methods ============
const navigateToOrders = () => {
  router.push('/orders');
};

const navigateToCalenders = () => {
  router.push('/scheduler');
};

const handleLogout = () => {
  emit('logout');
};

// ============ Lifecycle ============
onMounted(async () => {
  try {
    const vehicles = await dbService.getVehicles();
    const orders = await dbService.getAllOrders();
    const transactions = await dbService.getTransactions();
    const maintenance = await dbService.getMaintenanceRecords();
    const expenses = await dbService.getExpenses();

    totalProducts.value = vehicles.length;
    pendingOrders.value = orders.filter(
        order => order.orderStatus === orderStatus.reserved
    ).length;
    todayOrders.value = orders.filter(
        order => new Date(order.createdAt).getDate() === new Date().getDate()
    ).length;
    todayRevenue.value = transactions
        .filter(order => new Date(order.created_at).getDate() === new Date().getDate())
        .reduce((sum, res) => sum + res.amount, 0);
    pendingMaintenance.value = maintenance.filter(
        m => m.payment_status === paymentStatus.pending
    ).length;
    pendingExpenses.value = expenses.filter(
        expense => expense.payment_status === paymentStatus.pending
    ).length;
  } catch (error) {
    console.error('Error loading dashboard data:', error);
  }
});
</script>

<style scoped>
:global(html),
:global(body),
:global(#app) {
  width: 100%;
  min-height: 100%;
  margin: 0;
}

:global(body) {
  background: var(--color-background-tertiary);
}

.dashboard {
  width: 100%;
  min-height: 100dvh;
  background: var(--color-background-tertiary);
  box-sizing: border-box;
  overflow-x: hidden;
}

.dashboard-content {
  width: 100%;
  min-height: calc(100dvh - 70px);
  padding: 2rem 0;
  box-sizing: border-box;
}

.dashboard-container {
  width: 100%;
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 2rem;
  display: flex;
  flex-direction: column;
  gap: 2.5rem;
  box-sizing: border-box;
}

/* =========================
   ACTION BUTTONS
========================= */

.action-row {
  display: flex;
  flex-wrap: wrap;
  gap: 1rem;
  margin-bottom: 1rem;
}

.make-order-button {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.75rem;
  padding: 1rem 2rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: var(--border-radius-md);
  font-size: 16px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s ease;
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
  min-width: 200px;
}

.make-order-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(102, 126, 234, 0.4);
}

.make-order-button:active {
  transform: translateY(0);
}

.make-order-button i {
  font-size: 20px;
}

/* =========================
   STATS
========================= */

.stats-grid {
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: 1.5rem;
}

.stat-card {
  min-width: 0;
  background: var(--color-background-primary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-lg);
  padding: 1.5rem;
  display: flex;
  align-items: flex-start;
  gap: 1.5rem;
  transition: all 0.2s ease;
  box-sizing: border-box;
}

.stat-card:hover {
  border-color: var(--color-border-secondary);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.06);
  transform: translateY(-2px);
}

.stat-icon {
  width: 56px;
  height: 56px;
  border-radius: var(--border-radius-md);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 28px;
  flex-shrink: 0;
  color: white;
}

.stat-icon.today {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.stat-icon.revenue {
  background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
}

.stat-icon.products {
  background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
}

.stat-icon.pending {
  background: linear-gradient(135deg, #fa709a 0%, #fee140 100%);
}

.stat-content {
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  flex: 1;
}

.stat-label {
  margin: 0;
  font-size: 13px;
  color: var(--color-text-secondary);
  font-weight: 400;
}

.stat-value {
  margin: 0;
  font-size: 24px;
  font-weight: 500;
  color: var(--color-text-primary);
  word-break: break-word;
}

/* =========================
   QUICK ACCESS
========================= */

.quick-links {
  width: 100%;
  padding: 1rem 0 2rem;
}

.disabled-link {
  pointer-events: none;
  opacity: 1;
  cursor: not-allowed;
}

.section-title {
  margin: 0 0 1.5rem 0;
  font-size: 20px;
  font-weight: 600;
  color: var(--color-text-primary);
}

/*
 * Instead of separate fixed rows, each .links-grid
 * independently responds to the available width.
 */
.links-grid {
  display: grid;
  grid-template-columns: repeat(5, minmax(0, 1fr));
  gap: 1rem;
  width: 100%;
  margin-bottom: 1rem;
}

/* Modern quick-access card */
.quick-link {
  position: relative;
  min-width: 0;
  min-height: 150px;

  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;

  gap: 1rem;
  padding: 1.5rem 1rem;

  background: var(--color-background-primary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: 18px;

  text-decoration: none;
  color: var(--color-text-primary);

  transition:
      transform 0.25s ease,
      box-shadow 0.25s ease,
      border-color 0.25s ease,
      background 0.25s ease;

  box-sizing: border-box;
  overflow: visible;
}

/* Action Required Badge */
.action-badge {
  position: absolute;
  top: -8px;
  right: -8px;

  padding: 0.4rem 0.75rem;
  background: linear-gradient(135deg, #f5576c 0%, #ff6b6b 100%);
  color: white;

  font-size: 11px;
  font-weight: 700;
  letter-spacing: 0.5px;
  text-transform: uppercase;

  border-radius: 12px;
  box-shadow: 0 4px 12px rgba(245, 87, 108, 0.35);

  white-space: nowrap;
  animation: badgePulse 2s ease-in-out infinite;
}

@keyframes badgePulse {
  0%, 100% {
    transform: scale(1);
    box-shadow: 0 4px 12px rgba(245, 87, 108, 0.35);
  }
  50% {
    transform: scale(1.08);
    box-shadow: 0 6px 16px rgba(245, 87, 108, 0.5);
  }
}

/* Small accent line at the top */
.quick-link::before {
  content: "";
  position: absolute;
  top: 0;
  left: 20%;
  right: 20%;
  height: 3px;

  background: linear-gradient(
      90deg,
      #667eea,
      #764ba2
  );

  border-radius: 0 0 10px 10px;

  opacity: 0;
  transform: scaleX(0.5);
  transition: all 0.25s ease;
}

/* Icon container */
.quick-link i {
  width: 58px;
  height: 58px;

  display: flex;
  align-items: center;
  justify-content: center;

  border-radius: 16px;

  font-size: 25px;
  color: #667eea;

  background: rgba(102, 126, 234, 0.10);

  transition:
      transform 0.25s ease,
      background 0.25s ease,
      color 0.25s ease;
}

/* Text */
.quick-link span:not(.action-badge) {
  display: block;
  width: 100%;

  font-size: 14px;
  font-weight: 600;
  line-height: 1.4;

  text-align: center;

  color: var(--color-text-primary);

  overflow-wrap: anywhere;
}

/* Hover */
.quick-link:hover {
  transform: translateY(-5px);

  border-color: rgba(102, 126, 234, 0.35);

  background: var(--color-background-secondary);

  box-shadow:
      0 10px 30px rgba(0, 0, 0, 0.08),
      0 3px 10px rgba(102, 126, 234, 0.08);
}

.quick-link:hover::before {
  opacity: 1;
  transform: scaleX(1);
}

.quick-link:hover i {
  transform: translateY(-2px) scale(1.05);

  color: white;

  background: linear-gradient(
      135deg,
      #667eea 0%,
      #764ba2 100%
  );

  box-shadow: 0 6px 15px rgba(102, 126, 234, 0.25);
}

/* Click effect */
.quick-link:active {
  transform: translateY(-1px);
}

/* Keyboard accessibility */
.quick-link:focus-visible {
  outline: 3px solid rgba(102, 126, 234, 0.3);
  outline-offset: 3px;
}

/* =========================
   LARGE TABLETS
========================= */

@media (max-width: 1100px) {
  .dashboard-container {
    padding: 0 1.5rem;
  }

  .stats-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }

  .links-grid {
    grid-template-columns: repeat(4, minmax(0, 1fr));
  }
}

/* =========================
   TABLETS
========================= */

@media (max-width: 850px) {
  .dashboard-content {
    padding: 1.5rem 0;
  }

  .dashboard-container {
    padding: 0 1.25rem;
    gap: 1.75rem;
  }

  .action-row {
    width: 100%;
  }

  .make-order-button {
    flex: 1;
    min-width: 0;
  }

  .links-grid {
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: 0.85rem;
  }

  .quick-link {
    min-height: 140px;
    padding: 1.25rem 0.75rem;
  }

  .quick-link i {
    width: 52px;
    height: 52px;
    font-size: 23px;
  }

  .quick-link span:not(.action-badge) {
    font-size: 13px;
  }

  .action-badge {
    font-size: 10px;
    padding: 0.35rem 0.6rem;
  }
}

/* =========================
   MOBILE
========================= */

@media (max-width: 600px) {
  .dashboard-content {
    padding: 1rem 0;
  }

  .dashboard-container {
    padding: 0 1rem;
    gap: 1.25rem;
  }

  /* Action buttons stack */
  .action-row {
    flex-direction: column;
    gap: 0.75rem;
    margin-bottom: 0.5rem;
  }

  .make-order-button {
    width: 100%;
    min-width: unset;
    padding: 0.9rem 1.25rem;
  }

  /* Stats */
  .stats-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 0.75rem;
  }

  .stat-card {
    padding: 1rem;
    flex-direction: column;
    align-items: flex-start;
    gap: 0.85rem;
  }

  .stat-icon {
    width: 46px;
    height: 46px;
    font-size: 21px;
  }

  .stat-label {
    font-size: 12px;
  }

  .stat-value {
    font-size: 20px;
  }

  /* Quick access */
  .quick-links {
    padding: 0.5rem 0 1.5rem;
  }

  .section-title {
    font-size: 18px;
    margin-bottom: 1rem;
  }

  .links-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 0.75rem;
    margin-bottom: 0.75rem;
  }

  .quick-link {
    min-height: 135px;
    padding: 1.25rem 0.65rem;
    border-radius: 15px;
    gap: 0.75rem;
  }

  .quick-link i {
    width: 48px;
    height: 48px;
    border-radius: 14px;
    font-size: 21px;
  }

  .quick-link span:not(.action-badge) {
    font-size: 12px;
    line-height: 1.35;
  }

  .action-badge {
    font-size: 9px;
    padding: 0.3rem 0.5rem;
    top: -6px;
    right: -6px;
  }
}

/* =========================
   SMALL PHONES
========================= */

@media (max-width: 380px) {
  .dashboard-container {
    padding: 0 0.75rem;
  }

  .stats-grid {
    gap: 0.6rem;
  }

  .stat-card {
    padding: 0.85rem;
  }

  .stat-icon {
    width: 42px;
    height: 42px;
    font-size: 19px;
  }

  .stat-value {
    font-size: 18px;
  }

  .links-grid {
    gap: 0.6rem;
  }

  .quick-link {
    min-height: 125px;
    padding: 1rem 0.5rem;
  }

  .quick-link i {
    width: 44px;
    height: 44px;
    font-size: 19px;
  }

  .quick-link span:not(.action-badge) {
    font-size: 11px;
  }

  .action-badge {
    font-size: 8px;
    padding: 0.25rem 0.4rem;
    top: -5px;
    right: -5px;
  }
}

/* =========================
   TOUCH DEVICES
========================= */

@media (hover: none) {
  .quick-link:hover {
    transform: none;
    box-shadow: none;
  }

  .quick-link:hover i {
    transform: none;
  }

  .make-order-button:hover {
    transform: none;
  }

  .action-badge {
    animation: none;
  }
}
</style>
