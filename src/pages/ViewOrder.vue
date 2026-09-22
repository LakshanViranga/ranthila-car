<template>
  <div class="reservations-page">
    <!-- Header Component -->
    <header-component
        :userName="`Logged in as ${loggedUser}`"
        :showBackButton="true"
        @logout="handleLogout"
    />

    <!-- Reservations Content -->
    <div class="reservations-content">
      <div class="reservations-container">

        <!-- Page Header -->
        <div class="page-header">
          <h1>Reservations</h1>
          <div class="header-actions">
            <v-select
                v-model="filterStatus"
                :items="filterOptions"
                item-title="label"
                item-value="value"
                class="filter-select"
                placeholder="Filter by status"
                density="compact"
                variant="outlined"
                hide-details
            />
          </div>
        </div>

        <!-- Reservations Table -->
        <div class="table-section">
          <div class="table-wrapper">
            <table class="reservations-table">
              <thead>
              <tr>
                <th>Reservation ID</th>
                <th>Customer Name</th>
                <th>Vehicle</th>
                <th>Total Amount</th>
                <th>Paid Amount</th>
                <th>Reservation Status</th>
                <th>Payment Status</th>
                <th>Actions</th>
              </tr>
              </thead>
              <tbody>
              <tr v-for="reservation in filteredReservations" :key="reservation.id" class="reservation-row">
                <td class="reservation-id">
                  <strong>#{{ reservation.reservationId }}</strong>
                </td>
                <td class="customer-name">
                  {{ reservation.customerName }}
                </td>
                <td class="vehicle">
                  {{ reservation.vehicleId }}
                </td>
                <td class="price">
                  {{ formatPrice(reservation.totalAmount) }}
                </td>
                <td class="price">
                  {{ formatPrice(reservation.paidAmount) }}
                </td>
                <td class="status">
                  <span :class="['status-badge', `status-${reservation.reservationStatus.toLowerCase()}`]">
                    {{ formatStatus(reservation.reservationStatus) }}
                  </span>
                </td>
                <td class="payment-status">
                  <span :class="['payment-badge', `payment-${convertSnakeCase(reservation.paymentStatus)}`]">
                    {{ formatPaymentStatus(reservation.paymentStatus) }}
                  </span>
                </td>
                <td class="actions">
                  <v-btn
                      v-if="reservation.reservationStatus === orderStatus.reserved"
                      icon
                      size="small"
                      variant="text"
                      class="btn-action view"
                      @click="viewReservation(reservation)"
                      title="View & Edit"
                  >
                    <v-icon icon="fa fa-edit" />
                  </v-btn>
                  <v-btn
                      v-if="reservation.reservationStatus === orderStatus.active"
                      icon
                      size="small"
                      variant="text"
                      class="btn-action complete"
                      @click="viewReservation(reservation)"
                      title="View & Complete"
                  >
                    <v-icon icon="fa fa-check" />
                  </v-btn>
                  <v-btn
                      v-if="reservation.reservationStatus === orderStatus.completed"
                      icon
                      size="small"
                      variant="text"
                      class="btn-action complete"
                      @click="viewReservation(reservation)"
                      title="View"
                  >
                    <v-icon icon="fa fa-eye" />
                  </v-btn>
                  <v-btn
                      icon
                      size="small"
                      variant="text"
                      class="btn-action delete"
                      @click="deleteReservation(reservation.id)"
                      title="Delete"
                  >
                    <v-icon icon="fa fa-trash" />
                  </v-btn>
                </td>
              </tr>
              </tbody>
            </table>

            <!-- Empty State -->
            <div v-if="filteredReservations.length === 0" class="empty-state">
              <v-icon icon="fa fa-inbox" />
              <p>No reservations found</p>
            </div>
          </div>
        </div>

        <!-- Summary Stats -->
        <div class="summary-stats">
          <div class="stat-card">
            <label>Total Reservations</label>
            <span class="stat-value">{{ reservations.length }}</span>
          </div>
          <div class="stat-card">
            <label>Active</label>
            <span class="stat-value active">{{ activeCount }}</span>
          </div>
          <div class="stat-card">
            <label>Completed</label>
            <span class="stat-value success">{{ completedCount }}</span>
          </div>
          <div class="stat-card">
            <label>Total Revenue</label>
            <span class="stat-value">{{ formatPrice(totalRevenue) }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import HeaderComponent from '../component/Header.vue'
import { dbService } from '../services/db.ts'
import {
  convertSnakeCase,
  orderStatus,
} from '../utils/constants.ts'
import { useAuthStore } from '../stores/auth.ts'
import { useSnackbar } from '../composables/useSnackbar.js'

const router = useRouter()
const { showSuccess, showError } = useSnackbar()
const authStore = useAuthStore()

// ============== AUTHENTICATION ==============
const loggedUser = ref(authStore.username)

// ============== FILTER & RESERVATIONS ==============
const filterStatus = ref('')
const reservations = ref([])

const filterOptions = [
  { label: 'All Reservations', value: '' },
  { label: 'Active', value: orderStatus.active },
  { label: 'Completed', value: orderStatus.completed },
  { label: 'Reserved', value: orderStatus.reserved },
]

// ============== COMPUTED PROPERTIES ==============
const filteredReservations = computed(() => {
  if (!filterStatus.value) return reservations.value
  return reservations.value.filter((res) => res.reservationStatus === filterStatus.value)
})

const activeCount = computed(() => {
  return reservations.value.filter((r) => r.reservationStatus === orderStatus.active).length
})

const completedCount = computed(() => {
  return reservations.value.filter((r) => r.reservationStatus === orderStatus.completed).length
})

const totalRevenue = computed(() => {
  return reservations.value.reduce((sum, res) => sum + res.paidAmount, 0)
})

// ============== METHODS ==============

/**
 * Format price to LKR currency
 */
const formatPrice = (price) => {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'LKR',
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(price || 0)
}

/**
 * Format status text
 */
const formatStatus = (status) => {
  return status.charAt(0).toUpperCase() + status.slice(1)
}

/**
 * Format payment status
 */
const formatPaymentStatus = (status) => {
  const statusMap = {
    paid: 'Paid',
    pending: 'Pending',
    cancelled: 'Cancelled',
    partial_paid: 'Partial Paid',
    not_paid: 'Not Paid',
  }
  return statusMap[status] || status
}

/**
 * View reservation - Navigate to detail page
 */
const viewReservation = (reservation) => {
  router.push({
    name: 'view-order-detail',
    params: { id: reservation.reservationId },
  })
}

/**
 * Delete reservation
 */
const deleteReservation = async (reservationId) => {
  if (!reservationId) {
    showError('Invalid reservation')
    return
  }

  try {
    await dbService.deleteOrder(reservationId)
    const index = reservations.value.findIndex((r) => r.id === reservationId)
    if (index > -1) {
      reservations.value.splice(index, 1)
    }
    showSuccess('Reservation deleted successfully!')
  } catch (error) {
    console.log('Error deleting reservation:', error)
    showError('Error deleting reservation. Please try again.')
  }
}

/**
 * Get customer image
 */
const getCustomerImage = (imageArray) => {
  if (imageArray && imageArray.length > 0) {
    const bytes = new Uint8Array(imageArray)
    const blob = new Blob([bytes], { type: 'image/jpg' })
    return URL.createObjectURL(blob)
  }
}

/**
 * Get all reservations
 */
const getReservations = async () => {
  try {
    const result = await dbService.getAllOrders()
    reservations.value = result.map((item) => ({
      id: item.id,
      orderNumber: item.orderNumber,
      reservationId: item.orderNumber,
      customerId: item.customerId,
      customerName: item.customerName,
      vehicleId: item.vehicleId,
      totalAmount: item.totalAmount,
      paidAmount: item.paidAmount,
      customerImage: item.customerImage,
      customerImagePreview: getCustomerImage(item.customerImage),
      guaranteeProperty: item.guaranteeProperty,
      guaranteeType: item.guaranteeType,
      reservationStatus: item.orderStatus,
      paymentType: item.paymentType,
      paymentStatus: item.paymentStatus,
      startTime: item.releaseTime,
      endTime: item.handoverTime,
      startMileage: item.startingMileage,
      endMileage: item.endMileage,
      notes: item.notes || '',
      bankTransferAmount: item.bankTransferAmount || 0,
      bankAccountName: item.bankAccountName || '',
      cashAmount: item.cashAmount || 0,
      discount: item.discount || 0,
    }))
    console.log('Reservations loaded successfully!', reservations.value)
  } catch (error) {
    console.log('Error fetching reservations:', error)
    showError('Error loading reservations')
  }
}

/**
 * Handle logout
 */
const handleLogout = () => {
  // Emit logout event if needed
}

// ============== LIFECYCLE ==============
onMounted(async () => {
  await getReservations()
})
</script>

<style scoped>
.reservations-page {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
  background: var(--color-background-tertiary);
}

.reservations-content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.reservations-container {
  display: flex;
  flex-direction: column;
  flex: 1;
  overflow: hidden;
  padding: 1.5rem;
  max-width: 1400px;
  margin: 0 auto;
  width: 100%;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
  gap: 1rem;
  flex-wrap: wrap;
}

.page-header h1 {
  margin: 0;
  font-size: 28px;
  font-weight: 500;
  color: var(--color-text-primary);
}

.header-actions {
  display: flex;
  gap: 1rem;
  min-width: 200px;
}

.filter-select {
  min-width: 200px;
}

/* === TABLE SECTION === */
.table-section {
  flex: 1;
  background: var(--color-background-primary);
  border-radius: var(--border-radius-lg);
  border: 1px solid var(--color-border-tertiary);
  padding: 1rem;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  margin-bottom: 1.5rem;
}

.table-wrapper {
  flex: 1;
  overflow-y: auto;
  overflow-x: auto;
  border-radius: 4px;
}

.reservations-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

.reservations-table thead {
  position: sticky;
  top: 0;
  background: var(--color-background-secondary);
  border-bottom: 1px solid var(--color-border-tertiary);
  z-index: 10;
}

.reservations-table th {
  padding: 0.75rem;
  text-align: left;
  font-weight: 500;
  color: var(--color-text-secondary);
  white-space: nowrap;
}

.reservations-table td {
  padding: 0.75rem;
  border-bottom: 1px solid var(--color-border-tertiary);
  vertical-align: middle;
}

.reservations-table tbody tr:hover {
  background: var(--color-background-secondary);
}

.reservation-id {
  font-family: monospace;
  color: var(--color-info);
  font-weight: 600;
}

.customer-name {
  color: var(--color-text-primary);
  font-weight: 500;
}

.vehicle {
  color: var(--color-text-secondary);
}

.price {
  text-align: left;
  font-family: monospace;
  color: var(--color-success);
  font-weight: 500;
}

.status {
  text-align: left;
}

.status-badge {
  display: inline-block;
  padding: 0.4rem 0.8rem;
  border-radius: 20px;
  font-size: 11px;
  font-weight: 500;
}

.status-active {
  background: #dbeafe;
  color: #1e40af;
}

.status-completed {
  background: #dcfce7;
  color: #16a34a;
}

.status-reserved {
  background: #fee2e2;
  color: #dc2626;
}

.payment-status {
  text-align: left;
}

.payment-badge {
  display: inline-block;
  padding: 0.4rem 0.8rem;
  border-radius: 20px;
  font-size: 11px;
  font-weight: 500;
}

.payment-paid {
  background: #dcfce7;
  color: #16a34a;
}

.payment-partial_paid {
  background: #fef3c7;
  color: #d97706;
}

.payment-not_paid {
  background: #fee2e2;
  color: #dc2626;
}

.actions {
  display: flex;
  gap: 0.5rem;
  justify-content: left;
  white-space: nowrap;
}

.btn-action {
  transition: all var(--transition-fast);
  cursor: pointer;
}

.btn-action.view:hover {
  color: var(--color-info);
}

.btn-action.complete:hover {
  color: var(--color-success);
}

.btn-action.delete:hover {
  color: var(--color-danger);
}

.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 3rem;
  color: var(--color-text-secondary);
  gap: 1rem;
}

.empty-state i {
  font-size: 64px;
  opacity: 0.3;
}

/* === SUMMARY STATS === */
.summary-stats {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
}

.stat-card {
  background: var(--color-background-primary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-lg);
  padding: 1.25rem;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.stat-card label {
  font-size: 13px;
  color: var(--color-text-secondary);
  font-weight: 400;
}

.stat-value {
  font-size: 24px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.stat-value.active {
  color: #1e40af;
}

.stat-value.success {
  color: var(--color-success);
}

/* === RESPONSIVE === */
@media (max-width: 1024px) {
  .reservations-container {
    padding: 1rem;
  }

  .reservations-table {
    font-size: 12px;
  }

  .reservations-table th,
  .reservations-table td {
    padding: 0.6rem;
  }
}

@media (max-width: 768px) {
  .page-header {
    flex-direction: column;
    align-items: flex-start;
  }

  .header-actions {
    width: 100%;
  }

  .filter-select {
    width: 100%;
    min-width: auto;
  }

  .reservations-table {
    font-size: 11px;
  }

  .reservations-table th,
  .reservations-table td {
    padding: 0.5rem;
  }

  .summary-stats {
    grid-template-columns: repeat(2, 1fr);
  }
}

/* Scrollbar styling */
.table-wrapper::-webkit-scrollbar {
  width: 6px;
}

.table-wrapper::-webkit-scrollbar-track {
  background: transparent;
}

.table-wrapper::-webkit-scrollbar-thumb {
  background: var(--color-border-secondary);
  border-radius: 3px;
}

.table-wrapper::-webkit-scrollbar-thumb:hover {
  background: var(--color-border-primary);
}

:deep(.v-field__input) {
  padding-left: 0.75rem !important;
  padding-right: 0.75rem !important;
}

:deep(.v-select__content) {
  padding-left: 0.75rem !important;
  padding-right: 0.75rem !important;
}
</style>
