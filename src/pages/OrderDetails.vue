<template>
  <div v-if="reservation" class="reservation-detail-page">
    <!-- Header Component -->
    <header-component
        :userName="`Logged in as ${loggedUser}`"
        :showBackButton="true"
        @logout="handleLogout"
    />

    <!-- Detail Content -->
    <div class="detail-content">
      <div class="detail-container">
        <!-- Back Button & Title -->
        <div class="detail-header">
          <v-btn
              icon
              variant="text"
              color="info"
              @click="goBack"
              class="back-btn"
          >
            <v-icon icon="fa fa-arrow-left" />
          </v-btn>
          <h1>Reservation #{{ reservation?.reservationId }}</h1>
          <v-spacer />
          <div class="header-status">
            <span :class="['status-badge', `status-${convertSnakeCase(reservation?.reservationStatus)}`]">
              {{ formatStatus(reservation?.reservationStatus) }}
            </span>
          </div>
        </div>

        <!-- Main Content -->
        <div class="detail-main">
          <!-- Customer & Vehicle Info -->
          <div class="info-section">
            <h3 class="section-title">Reservation Information</h3>
            <div class="info-grid">
              <div class="info-item">
                <label>Customer Name</label>
                <p>{{ reservation?.customerName }}</p>
              </div>
              <div class="info-item">
                <label>Vehicle</label>
                <p>{{ reservation?.vehicleId }}</p>
              </div>
              <div class="info-item">
                <label>Reservation Status</label>
                <span :class="['status-badge', `status-${convertSnakeCase(reservation?.reservationStatus)}`]">
                  {{ formatStatus(reservation?.reservationStatus) }}
                </span>
              </div>
              <div class="info-item">
                <label>Payment Status</label>
                <span :class="['payment-badge', `payment-${convertSnakeCase(reservation?.paymentStatus)}`]">
                  {{ formatPaymentStatus(reservation?.paymentStatus) }}
                </span>
              </div>
            </div>
          </div>

          <!-- Rental Details -->
          <div class="edit-section">
            <h3 class="section-title">Rental Details</h3>
            <div class="form-grid">
              <!-- Start Date/Time Picker -->
              <div class="form-group">
                <label>Start Date & Time <span class="required">*</span></label>
                <div class="datetime-picker-wrapper">
                  <v-text-field
                      v-model="startDateTimeDisplay"
                      label="Start Date & Time"
                      placeholder="Select date and time"
                      density="compact"
                      variant="outlined"
                      readonly
                      class="datetime-field"
                      hide-details
                  />
                  <v-btn
                      icon
                      size="small"
                      variant="tonal"
                      color="info"
                      @click="showStartDateTimePicker = true"
                      class="calendar-btn"
                      :disabled="reservation?.reservationStatus === orderStatus.completed"
                  >
                    <v-icon icon="fa fa-calendar" />
                  </v-btn>
                </div>
              </div>

              <!-- End Date/Time Picker -->
              <div v-if="reservation?.reservationStatus !== orderStatus.reserved" class="form-group">
                <label>End Date & Time</label>
                <div class="datetime-picker-wrapper">
                  <v-text-field
                      v-model="endDateTimeDisplay"
                      label="End Date & Time"
                      placeholder="Select date and time"
                      density="compact"
                      variant="outlined"
                      readonly
                      class="datetime-field"
                      hide-details
                  />
                  <v-btn
                      icon
                      size="small"
                      variant="tonal"
                      color="info"
                      @click="showEndDateTimePicker = true"
                      class="calendar-btn"
                      :disabled="reservation?.reservationStatus === orderStatus.completed"
                  >
                    <v-icon icon="fa fa-calendar" />
                  </v-btn>
                </div>
              </div>

              <!-- Start Mileage -->
              <div class="form-group">
                <label for="startMileage">Start Mileage (km)<span class="required">*</span></label>
                <v-text-field
                    id="startMileage"
                    v-model.number="reservation.startMileage"
                    type="number"
                    placeholder="0"
                    density="compact"
                    variant="outlined"
                    :readonly="reservation?.reservationStatus === orderStatus.completed"
                />
              </div>

              <!-- End Mileage -->
              <div v-if="reservation?.reservationStatus !== orderStatus.reserved" class="form-group">
                <label for="endMileage">End Mileage (km)</label>
                <v-text-field
                    id="endMileage"
                    v-model.number="reservation.endMileage"
                    type="number"
                    placeholder="0"
                    density="compact"
                    variant="outlined"
                    :readonly="reservation?.reservationStatus === orderStatus.completed"
                    :rules="[endMileageValidation]"
                />
              </div>

              <!-- Total Distance (Read-only) -->
              <div v-if="reservation?.reservationStatus !== orderStatus.reserved" class="form-group">
                <label for="totalDistance">Total Distance (km)</label>
                <v-text-field
                    id="totalDistance"
                    :value="calculateTotalDistance()"
                    type="number"
                    readonly
                    density="compact"
                    variant="outlined"
                    hide-details
                />
              </div>

              <!-- Advanced Payment -->
              <div v-if="reservation?.reservationStatus === orderStatus.reserved" class="form-group">
                <label for="advancedPayment">Advanced Payment (LKR)</label>
                <v-text-field
                    id="advancedPayment"
                    v-model.number="advancedPaymentAmount"
                    type="number"
                    placeholder="0.00"
                    density="compact"
                    variant="outlined"
                    hide-details
                    step="0.01"
                />
              </div>

              <!-- Guarantee Type -->
              <div class="form-group">
                <label for="guaranteeType">Guarantee Property Type<span v-if="reservation?.reservationStatus === orderStatus.reserved" class="required">*</span></label>
                <v-select
                    v-model="reservation.guaranteeType"
                    :items="guaranteePropertyType"
                    item-title="title"
                    item-value="value"
                    class="payment-select"
                    density="compact"
                    variant="outlined"
                    placeholder="Select guarantee property type"
                    :readonly="reservation?.reservationStatus !== orderStatus.reserved"
                />
              </div>

              <!-- Guarantee Details -->
              <div class="form-group">
                <label for="guaranteeProperty">Guarantee Property</label>
                <v-text-field
                    id="guaranteeProperty"
                    v-model="reservation.guaranteeProperty"
                    type="text"
                    placeholder="Enter guarantee property"
                    density="compact"
                    variant="outlined"
                    :readonly="reservation?.reservationStatus !== orderStatus.reserved"
                    hide-details
                />
              </div>

              <!-- Payment Type (for active reservations) -->
              <div v-if="reservation?.reservationStatus === orderStatus.active" class="form-group">
                <label>Payment Type</label>
                <v-select
                    v-model="reservation.paymentType"
                    :items="paymentTypeOptions"
                    item-title="label"
                    item-value="value"
                    placeholder="Select payment type"
                    density="compact"
                    variant="outlined"
                    hide-details
                />
              </div>
            </div>

            <!-- Notes -->
            <div class="form-group full-width">
              <label for="notes">Notes</label>
              <v-textarea
                  id="notes"
                  v-model="reservation.notes"
                  placeholder="Add any notes about the reservation"
                  rows="3"
                  density="compact"
                  variant="outlined"
                  hide-details
                  :readonly="reservation?.reservationStatus === orderStatus.completed"
              />
            </div>

            <!-- Customer Image -->
            <div class="form-group full-width">
              <label>Customer Image With Vehicle <span class="required">*</span></label>
              <div class="image-upload-section">
                <div v-if="reservation.customerImagePreview" class="image-preview">
                  <img :src="reservation.customerImagePreview" alt="Customer Image" class="preview-img" />
                  <v-btn
                      v-if="reservation?.reservationStatus !== orderStatus.completed"
                      icon
                      size="small"
                      variant="text"
                      color="error"
                      @click="clearImage('customerImagePreview')"
                      class="remove-btn"
                  >
                    <v-icon icon="fa fa-trash" />
                  </v-btn>
                </div>
                <div v-else-if="reservation?.reservationStatus !== orderStatus.completed" class="upload-area" @click="customerImageInput?.click()">
                  <input
                      ref="customerImageInput"
                      type="file"
                      accept="image/*"
                      style="display: none"
                      @change="(e) => handleImageUpload(e, 'customerImage', 'customerImagePreview')"
                  />
                  <v-icon icon="fa fa-camera" class="upload-icon" />
                  <p class="upload-text">Click to upload customer image with vehicle</p>
                  <p class="upload-hint">JPG, PNG (max 5MB)</p>
                </div>
              </div>
            </div>
          </div>

          <!-- Summary -->
          <div class="summary-section">
            <h3 class="section-title">Billing Summary</h3>
            <div class="summary-card">
              <div v-if="reservation?.reservationStatus === orderStatus.completed || reservation?.reservationStatus === orderStatus.active" class="summary-row">
                <label>Total Distance</label>
                <span>{{ calculateTotalDistance() }} km</span>
              </div>
              <div v-if="reservation?.reservationStatus !== orderStatus.reserved" class="summary-row">
                <label>Duration</label>
                <span>{{ calculateDuration() }}</span>
              </div>
              <div class="summary-row">
                <label>Total Amount</label>
                <span v-if="reservation?.reservationStatus !== orderStatus.completed">{{ calculateTotalAmount() }}</span>
                <span v-else>{{ formatPrice(reservation?.totalAmount) }}</span>
              </div>
              <div class="summary-row">
                <label>Paid Amount</label>
                <span>{{ formatPrice(reservation?.paidAmount || 0) }}</span>
              </div>
              <div class="summary-row final">
                <label>Balance Amount</label>
                <span v-if="reservation?.reservationStatus !== orderStatus.completed" class="amount">{{ formatPrice(calculateBalancePayment()) }}</span>
                <span v-else>{{ formatPrice(reservation?.totalAmount - reservation?.paidAmount) }}</span>
              </div>
            </div>

            <!-- Payment Details Section -->
            <div v-if="reservation?.reservationStatus === orderStatus.active || reservation?.reservationStatus === orderStatus.completed" class="payment-details-section">
              <h3 class="section-title">Final Payment Details</h3>
              <div class="form-grid">
                <!-- Bank Transfer Amount -->
                <div class="form-group">
                  <label for="bankTransferAmount">Bank Transfer Amount (LKR)</label>
                  <v-text-field
                      id="bankTransferAmount"
                      v-model.number="reservation.bankTransferAmount"
                      type="number"
                      placeholder="0.00"
                      density="compact"
                      variant="outlined"
                      hide-details
                      step="0.01"
                      :readonly="reservation?.reservationStatus === orderStatus.completed"
                  />
                </div>

                <!-- Bank Account Name -->
                <div class="form-group">
                  <label for="bankAccountName">Bank Account Name</label>
                  <v-text-field
                      id="bankAccountName"
                      v-model="reservation.bankAccountName"
                      type="text"
                      placeholder="e.g., ABC Bank - Current Account"
                      density="compact"
                      variant="outlined"
                      hide-details
                      :readonly="reservation?.reservationStatus === orderStatus.completed"
                  />
                </div>

                <!-- Cash Amount -->
                <div class="form-group">
                  <label for="cashAmount">Cash Amount (LKR)</label>
                  <v-text-field
                      id="cashAmount"
                      v-model.number="reservation.cashAmount"
                      type="number"
                      placeholder="0.00"
                      density="compact"
                      variant="outlined"
                      hide-details
                      step="1"
                      :readonly="reservation?.reservationStatus === orderStatus.completed"
                  />
                </div>

                <!-- Discount Amount -->
                <div class="form-group">
                  <label for="discountAmount">Discount Amount (LKR)</label>
                  <v-text-field
                      id="discountAmount"
                      v-model.number="reservation.discount"
                      type="number"
                      placeholder="0.00"
                      density="compact"
                      variant="outlined"
                      hide-details
                      step="1"
                      :readonly="reservation?.reservationStatus === orderStatus.completed"
                  />
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Action Footer -->
        <div class="detail-footer">
          <v-btn
              variant="outlined"
              @click="goBack"
          >
            Back to List
          </v-btn>
          <v-spacer />
          <v-btn
              v-if="reservation?.reservationStatus === orderStatus.active"
              color="error"
              variant="flat"
              prepend-icon="fa fa-check-circle"
              @click="completeReservation"
          >
            Save and Complete
          </v-btn>
          <v-btn
              v-if="reservation?.reservationStatus === orderStatus.reserved"
              color="success"
              variant="flat"
              prepend-icon="fa fa-save"
              @click="isConfirmStartOrder"
          >
            Save and Active
          </v-btn>
        </div>
      </div>
    </div>

    <!-- Date Time Picker Dialog - Start -->
    <v-dialog
        v-model="showStartDateTimePicker"
        max-width="350px"
        persistent
    >
      <v-card class="datetime-picker-card">
        <v-card-title class="picker-header">
          <span>Select Start Date & Time</span>
          <v-spacer />
          <v-btn
              icon
              size="small"
              variant="text"
              color="white"
              @click="showStartDateTimePicker = false"
          >
            <v-icon icon="fa fa-times" />
          </v-btn>
        </v-card-title>

        <v-card-text class="picker-content">
          <!-- Date Picker -->
          <div class="picker-section">
            <label>Date</label>
            <v-date-picker
                v-model="startDatePicker"
                class="date-picker"
            />
          </div>

          <!-- Hour Picker -->
          <div class="picker-section">
            <label>Hour (6-20)</label>
            <div class="hour-picker">
              <v-btn
                  icon
                  size="small"
                  variant="tonal"
                  @click="startHour = startHour > 6 ? startHour - 1 : 6"
              >
                <v-icon icon="fa fa-minus" />
              </v-btn>
              <div class="hour-display">{{ String(startHour).padStart(2, '0') }}:00</div>
              <v-btn
                  icon
                  size="small"
                  variant="tonal"
                  @click="startHour = startHour < 20 ? startHour + 1 : 20"
              >
                <v-icon icon="fa fa-plus" />
              </v-btn>
            </div>
          </div>

          <!-- Quick Hour Selection -->
          <div class="quick-hours">
            <v-btn
                v-for="h in quickHours"
                :key="h"
                size="small"
                :variant="startHour === h ? 'flat' : 'tonal'"
                :color="startHour === h ? 'info' : 'default'"
                @click="startHour = h"
                class="hour-btn"
            >
              {{ String(h).padStart(2, '0') }}
            </v-btn>
          </div>
        </v-card-text>

        <v-card-actions class="picker-footer">
          <v-btn
              variant="outlined"
              @click="showStartDateTimePicker = false"
          >
            Cancel
          </v-btn>
          <v-spacer />
          <v-btn
              color="info"
              variant="flat"
              @click="confirmStartDateTime"
          >
            Confirm
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- Date Time Picker Dialog - End -->
    <v-dialog
        v-model="showEndDateTimePicker"
        max-width="350px"
        persistent
    >
      <v-card class="datetime-picker-card">
        <v-card-title class="picker-header">
          <span>Select End Date & Time</span>
          <v-spacer />
          <v-btn
              icon
              size="small"
              variant="text"
              color="white"
              @click="showEndDateTimePicker = false"
          >
            <v-icon icon="fa fa-times" />
          </v-btn>
        </v-card-title>

        <v-card-text class="picker-content">
          <!-- Date Picker -->
          <div class="picker-section">
            <label>Date</label>
            <v-date-picker
                v-model="endDatePicker"
                class="date-picker"
            />
          </div>

          <!-- Hour Picker -->
          <div class="picker-section">
            <label>Hour (0-23)</label>
            <div class="hour-picker">
              <v-btn
                  icon
                  size="small"
                  variant="tonal"
                  @click="endHour = endHour > 0 ? endHour - 1 : 0"
              >
                <v-icon icon="fa fa-minus" />
              </v-btn>
              <div class="hour-display">{{ String(endHour).padStart(2, '0') }}:00</div>
              <v-btn
                  icon
                  size="small"
                  variant="tonal"
                  @click="endHour = endHour < 23 ? endHour + 1 : 23"
              >
                <v-icon icon="fa fa-plus" />
              </v-btn>
            </div>
          </div>

          <!-- Quick Hour Selection -->
          <div class="quick-hours">
            <v-btn
                v-for="h in quickHours"
                :key="h"
                size="small"
                :variant="endHour === h ? 'flat' : 'tonal'"
                :color="endHour === h ? 'info' : 'default'"
                @click="endHour = h"
                class="hour-btn"
            >
              {{ String(h).padStart(2, '0') }}
            </v-btn>
          </div>
        </v-card-text>

        <v-card-actions class="picker-footer">
          <v-btn
              variant="outlined"
              @click="showEndDateTimePicker = false"
          >
            Cancel
          </v-btn>
          <v-spacer />
          <v-btn
              color="info"
              variant="flat"
              @click="confirmEndDateTime"
          >
            Confirm
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
    <ConfirmationModal ref="confirmDialog" />
  </div>
  <div v-else>
    <v-progress-circular
        indeterminate
        color="info"
    />
    <span>Loading reservation...</span>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import HeaderComponent from '../component/Header.vue'
import { dbService } from '../services/db.ts'
import {
  convertSnakeCase,
  orderStatus,
  paymentStatus,
  paymentTypes,
  guaranteePropertyType
} from '../utils/constants.ts'
import { commonUtils } from "../utils/common.ts"
import { useAuthStore } from '../stores/auth.ts'
import { useSnackbar } from '../composables/useSnackbar.js'
import ConfirmationModal from "../component/ConfirmationModal.vue";

const router = useRouter()
const route = useRoute()
const { showSuccess, showError } = useSnackbar()
const authStore = useAuthStore()

// ============== AUTHENTICATION ==============
const loggedUser = ref(authStore.username)

// ============== MODAL STATE ==============
const showStartDateTimePicker = ref(false)
const showEndDateTimePicker = ref(false)

// ============== RESERVATION STATE ==============
const reservation = ref(null)
const vehicles = ref([])
const advancedPaymentAmount = ref(0)
const totalDistance = ref(0)
const totalAmount = ref(0)
const totalHours = ref(0)

// ============== FILE INPUT REFS ==============
const customerImageInput = ref(null)

// ============== DATE TIME PICKER STATE ==============
const startDatePicker = ref(new Date())
const startHour = ref(6)
const endDatePicker = ref(new Date())
const endHour = ref(22)
const quickHours = ref([6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22])

const paymentTypeOptions = [
  { label: 'Cash', value: paymentTypes.cashPayment },
  { label: 'Credit', value: paymentTypes.creditPayment },
]

const confirmDialog = ref(null)

// ============== COMPUTED PROPERTIES ==============
const startDateTimeDisplay = computed(() => {
  try {
    if (!reservation.value?.startTime) return 'Not set'
    const date = new Date(reservation.value.startTime)
    if (isNaN(date.getTime())) return 'Invalid date'
    return date.toLocaleString('en-US', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
      hour12: false,
    })
  } catch (e) {
    return 'Invalid date'
  }
})

const endDateTimeDisplay = computed(() => {
  try {
    if (!reservation.value?.endTime) return 'Not set'
    const date = new Date(reservation.value.endTime)
    if (isNaN(date.getTime())) return 'Invalid date'
    return date.toLocaleString('en-US', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
      hour12: false,
    })
  } catch (e) {
    return 'Invalid date'
  }
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
 * Calculate total distance
 */
const calculateTotalDistance = () => {
  if (reservation.value?.endMileage && reservation.value?.startMileage) {
    totalDistance.value = reservation.value.endMileage - reservation.value.startMileage
    return totalDistance.value
  }
  return 0
}

/**
 * Calculate duration
 */
const calculateDuration = () => {
  if (!reservation.value?.startTime || !reservation.value?.endTime) {
    return '-'
  }
  const start = new Date(reservation.value.startTime)
  const end = new Date(reservation.value.endTime)
  const diffMs = end - start
  const diffHours = Math.floor(diffMs / (1000 * 60 * 60))
  totalHours.value = diffHours
  return `${diffHours}h`
}

/**
 * Calculate total amount
 */
const calculateTotalAmount = () => {
  if (
      reservation.value?.endMileage &&
      reservation.value?.startMileage
  ) {
    const orderedVehicles = vehicles.value.find((v) => {
      return v.vehicle_id === reservation.value.vehicleId
    })
    const totalDays = Math.round(totalHours.value / 24)
    const waiveOffDistance = orderedVehicles.distance_range * totalDays
    if (totalDistance.value > waiveOffDistance) {
      totalAmount.value = (totalDistance.value - waiveOffDistance) * orderedVehicles.unit_price + orderedVehicles.base_price * totalDays
    } else {
      totalAmount.value = totalDays > 0 ? orderedVehicles.base_price * totalDays : orderedVehicles.base_price
    }
    return formatPrice(totalAmount.value)
  }
  totalAmount.value = reservation.value?.totalAmount
  return formatPrice(reservation.value?.totalAmount || 0)
}

const calculateBalancePayment = () => {
  return totalAmount.value - reservation.value?.paidAmount - advancedPaymentAmount.value
}

/**
 * Confirm start date and time
 */
const confirmStartDateTime = () => {
  if (!startDatePicker.value) {
    showError('Please select a date')
    return
  }
  const dateTimeString = `${commonUtils.formatLocalDate(startDatePicker.value)}T${String(startHour.value).padStart(2, '0')}:00`
  reservation.value.startTime = dateTimeString
  showStartDateTimePicker.value = false
}

/**
 * Confirm end date and time
 */
const confirmEndDateTime = () => {
  if (!endDatePicker.value) {
    showError('Please select a date')
    return
  }
  const dateTimeString = `${commonUtils.formatLocalDate(endDatePicker.value)}T${String(endHour.value).padStart(2, '0')}:00`
  reservation.value.endTime = dateTimeString
  showEndDateTimePicker.value = false
}

/**
 * Handle image upload
 */
const handleImageUpload = async (event, fieldName, previewFieldName) => {
  try {
    const file = event.target.files?.[0]
    if (!file) return

    const maxSize = 5 * 1024 * 1024
    if (file.size > maxSize) {
      showError('Image size must be less than 5MB')
      return
    }

    const allowedTypes = ['image/jpeg', 'image/png', 'image/jpg']
    if (!allowedTypes.includes(file.type)) {
      showError('Please upload JPG or PNG image only')
      return
    }

    const reader = new FileReader()
    reader.onload = (e) => {
      try {
        const result = e.target?.result

        if (!(result instanceof ArrayBuffer)) {
          showError('Failed to read file')
          return
        }

        const byteArray = new Uint8Array(result)

        if (reservation.value) {
          reservation.value[fieldName] = Array.from(byteArray)

          const blob = new Blob([byteArray], { type: file.type })
          reservation.value[previewFieldName] = URL.createObjectURL(blob)
        }
      } catch (err) {
        console.log('Error processing image:', err)
        showError('Error processing image')
      }
    }

    reader.onerror = () => {
      showError('Error reading file')
    }

    reader.readAsArrayBuffer(file)
    event.target.value = ''
  } catch (error) {
    console.log('Error uploading image:', error)
    showError('Error uploading image')
  }
}

/**
 * Clear image
 */
const clearImage = (fieldName) => {
  if (!reservation.value) return

  reservation.value[fieldName] = null
  const imageFieldName = fieldName.replace('Preview', '')
  reservation.value[imageFieldName] = null
}

const isConfirmStartOrder = async () => {
  if (reservation.value?.startMileage > 0) {
    confirmDialog.value.open({
      title: 'Active Order',
      subtitle: 'This action cannot be undone',
      message: 'Are you sure you want to active this order?',
      type: 'warning',
      confirmText: 'Yes',
      onConfirm: async () => {
        await updateReservation()
      },
    })
  } else {
    await updateReservation()
  }
}
/**
 * Update reservation
 */
const updateReservation = async () => {
  try {
    const updateData = {
      order_number: reservation.value.reservationId,
      vehicle_id: reservation.value.vehicleId,
      customer_id: reservation.value.customerId,
      starting_mileage: reservation.value.startMileage,
      release_time: reservation.value.startTime,
      guarantee_type: reservation.value.guaranteeType,
      guarantee_property: reservation.value.guaranteeProperty,
      customer_image: reservation.value.customerImage || [],
      total_amount: reservation.value.totalAmount,
      total_distance: 0,
      advanced_payment: advancedPaymentAmount.value,
      payment_type: reservation.value.paymentType,
      payment_status: advancedPaymentAmount.value || reservation.value.paidAmount > 0
          ? paymentStatus.partialPaid
          : paymentStatus.notPaid,
      paid_amount: reservation.value.paidAmount + advancedPaymentAmount.value,
      order_status: reservation.value.startMileage > 0 ? orderStatus.active : orderStatus.reserved,
      notes: reservation.value.notes,
      updated_by: loggedUser.value,
    }

    await dbService.updateOrder(updateData)

    if (updateData.advanced_payment > 0) {
      await dbService.addTransaction({
        order_number: updateData.order_number,
        vehicle_id: updateData.vehicle_id,
        amount: updateData.advanced_payment,
        payment_type: paymentTypes.cashPayment,
        created_by: loggedUser.value,
      })
    }

    showSuccess('Reservation updated successfully!')
    goBack()
  } catch (error) {
    console.log('Error updating reservation:', error)
    showError('Error updating reservation. Please try again.')
  }
}

/**
 * Complete reservation
 */
const completeReservation = async () => {
  confirmDialog.value.open({
    title: 'Completed Order',
    subtitle: 'This will be effect to financial and cannot be undone!',
    message: 'Are you sure you want to complete the order?',
    type: 'warning',
    confirmText: 'Yes',
    onConfirm: async () => {

      try {
        const updateData = {
          order_number: reservation.value.reservationId,
          vehicle_id: reservation.value.vehicleId,
          customer_id: reservation.value.customerId,
          starting_mileage: reservation.value.startMileage,
          end_mileage: reservation.value.endMileage,
          release_time: reservation.value.startTime,
          handover_time: reservation.value.endTime,
          guarantee_type: reservation.value.guaranteeType,
          guarantee_property: reservation.value.guaranteeProperty,
          customer_image: reservation.value.customerImage || [],
          total_distance: totalDistance.value,
          total_amount: totalAmount.value,
          advanced_payment: 0,
          payment_type: reservation.value.paymentType,
          payment_method: paymentTypes.cashPayment,
          payment_status: paymentStatus.paid,
          paid_amount: reservation.value.paidAmount + reservation.value.bankTransferAmount + reservation.value.cashAmount,
          order_status: orderStatus.completed,
          notes: reservation.value.notes,
          bank_transfer_amount: reservation.value.bankTransferAmount || 0,
          bank_account_name: reservation.value.bankAccountName || null,
          cash_amount: reservation.value.cashAmount || 0,
          discount: reservation.value.discount || 0,
          updated_by: loggedUser.value,
        }

        await dbService.updateOrder(updateData)

        await dbService.addTransaction({
          order_number: updateData.order_number,
          vehicle_id: updateData.vehicle_id,
          amount: updateData.cash_amount,
          payment_type: paymentTypes.cashPayment,
          created_by: loggedUser.value,
        })

        if (reservation.value.bankTransferAmount > 0) {
          await dbService.addTransaction({
            order_number: updateData.order_number,
            vehicle_id: updateData.vehicle_id,
            amount: updateData.bank_transfer_amount,
            payment_type: paymentTypes.bankTransfer,
            created_by: loggedUser.value,
        })

        // update mileage
        await dbService.updateVehicleMilage({
          vehicle_id: updateData.vehicle_id,
          mileage: updateData.end_mileage
        })
      }

      showSuccess('Reservation completed successfully!')
       goBack()
      } catch (error) {
        console.log('Error completing reservation:', error)
        showError('Error completing reservation. Please try again.')
      }
    },
  })
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
 * Load reservation details
 */
const loadReservation = async () => {
  try {
    const reservationId = route.params.id
    if (!reservationId) {
      showError('Invalid reservation ID')
      goBack()
      return
    }

    // Fetch all reservations and find the specific one
    const allReservations = await dbService.getAllOrders()
    const foundReservation = allReservations.find((item) => item.orderNumber === reservationId)

    if (!foundReservation) {
      showError('Reservation not found')
      goBack()
      return
    }

    reservation.value = {
      id: foundReservation.id,
      orderNumber: foundReservation.orderNumber,
      reservationId: foundReservation.orderNumber,
      customerId: foundReservation.customerId,
      customerName: foundReservation.customerName,
      vehicleId: foundReservation.vehicleId,
      totalAmount: foundReservation.totalAmount,
      paidAmount: foundReservation.paidAmount,
      customerImage: foundReservation.customerImage,
      customerImagePreview: getCustomerImage(foundReservation.customerImage),
      guaranteeProperty: foundReservation.guaranteeProperty,
      guaranteeType: foundReservation.guaranteeType,
      reservationStatus: foundReservation.orderStatus,
      paymentType: foundReservation.paymentType,
      paymentStatus: foundReservation.paymentStatus,
      startTime: foundReservation.releaseTime,
      endTime: foundReservation.handoverTime,
      startMileage: foundReservation.startingMileage,
      endMileage: foundReservation.endMileage,
      notes: foundReservation.notes || '',
      bankTransferAmount: foundReservation.bankTransferAmount || 0,
      bankAccountName: foundReservation.bankAccountName || '',
      cashAmount: foundReservation.cashAmount || 0,
      discount: foundReservation.discount || 0,
    }

    // Initialize date/time picker
    if (reservation.value?.startTime) {
      try {
        const startDate = new Date(reservation.value.startTime)
        startDatePicker.value = startDate.toISOString().split('T')[0]
        startHour.value = startDate.getHours()
      } catch (e) {
        console.log('Error parsing start time:', e)
      }
    }

    if (reservation.value?.endTime) {
      try {
        const endDate = new Date(reservation.value.endTime)
        endDatePicker.value = endDate.toISOString().split('T')[0]
        endHour.value = endDate.getHours()
      } catch (e) {
        console.log('Error parsing end time:', e)
      }
    }
  } catch (error) {
    console.log('Error loading reservation:', error)
    showError('Error loading reservation details')
    goBack()
  }
}

/**
 * Get vehicles
 */
const getVehicles = async () => {
  try {
    vehicles.value = await dbService.getVehicles()
  } catch (error) {
    console.log('Error fetching vehicles:', error)
    showError('Error loading vehicles')
  }
}

/**
 * Navigate back
 */
const goBack = () => {
  router.back()
}

/**
 * Handle logout
 */
const handleLogout = () => {
  // Emit logout event if needed
}

// Field Validation
const endMileageValidation = (input) => {
  if (input === null || input === undefined){
    return true
  }

  if (reservation.value.startMileage && input > reservation.value.startMileage){
    return true
  } else {
    return 'End mileage should greater than starting mileage'
  }
}

// ============== LIFECYCLE ==============
onMounted(async () => {
  await getVehicles()
  await loadReservation()
})
</script>

<style scoped>

/* === ROOT PAGE CONTAINER === */
.reservation-detail-page {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
  background: linear-gradient(135deg, #f0f4f8 0%, #53a5e1 100%);
}

.detail-content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.detail-container {
  display: flex;
  flex-direction: column;
  flex: 1;
  overflow: hidden;
  padding: 2rem 1.5rem;
  max-width: 1000px;
  margin: 0 auto;
  width: 100%;
}

/* ================================================================
   HEADER SECTION
   ================================================================ */
.detail-header {
  display: flex;
  align-items: center;
  gap: 1.5rem;
  margin-bottom: 2rem;
}

.detail-header h1 {
  margin: 0;
  font-size: 32px;
  font-weight: 700;
  color: #1f2937;
  letter-spacing: -0.5px;
}

.back-btn {
  margin-right: 0.5rem;
  color: #6b7280;
  transition: color 0.2s ease;
}

.back-btn:hover {
  color: #1e40af;
}

.header-status {
  display: flex;
  gap: 1rem;
  margin-left: auto;
}

/* Status Badges - Better styling with borders for clarity */
.status-badge {
  display: inline-flex;
  align-items: center;
  padding: 0.5rem 1rem;
  border-radius: 6px;
  font-size: 12px;
  font-weight: 600;
  letter-spacing: 0.3px;
  text-transform: capitalize;
}

.status-active {
  background: #dbeafe;
  color: #0c4a6e;
  border: 1px solid #7dd3fc;
}

.status-completed {
  background: #dcfce7;
  color: #15803d;
  border: 1px solid #86efac;
}

.status-reserved {
  background: #fee2e2;
  color: #7f1d1d;
  border: 1px solid #fca5a5;
}

/* ================================================================
   MAIN CONTENT AREA
   ================================================================ */
.detail-main {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  display: flex;
  flex-direction: column;
  gap: 2.5rem;
  margin-bottom: 1.5rem;
  padding-right: 0.5rem;
}

/* Improved scrollbar styling */
.detail-main::-webkit-scrollbar {
  width: 8px;
}

.detail-main::-webkit-scrollbar-track {
  background: transparent;
}

.detail-main::-webkit-scrollbar-thumb {
  background: #d1d5db;
  border-radius: 4px;
  transition: background 0.2s ease;
}

.detail-main::-webkit-scrollbar-thumb:hover {
  background: #9ca3af;
}

/* ================================================================
   SECTION TITLE
   ================================================================ */
.section-title {
  margin: 0 0 1.25rem 0;
  font-size: 14px;
  font-weight: 700;
  color: #1f2937;
  letter-spacing: 0.3px;

}

/* ================================================================
   INFO SECTION (Read-only fields)
   ================================================================ */
.info-section {
  background: linear-gradient(135deg, #f9fbfd 0%, #f5f8fc 100%);
  padding: 1.5rem;
  border-radius: 8px;
  border: 1px solid #e0e7f1;
  box-shadow: 0 2px 8px rgba(30, 64, 175, 0.08);
}

.info-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 2rem;
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.info-item label {
  font-size: 11px;
  color: #6b7280;
  font-weight: 700;
  letter-spacing: 0.5px;
  /* Removed: text-transform: uppercase; */
  /* Using weight & letter-spacing for visual distinction instead */
}

.info-item p {
  margin: 0;
  font-size: 15px;
  color: #1f2937;
  font-weight: 500;
}

/* ================================================================
   EDIT SECTION (Form fields)
   ================================================================ */
.edit-section {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  background: linear-gradient(135deg, #f8fafd 0%, #f3f7fc 100%);
  padding: 1.5rem;
  border-radius: 8px;
  border: 1px solid #dce4f0;
  box-shadow: 0 2px 8px rgba(30, 64, 175, 0.08);
}

.form-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1.5rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.form-group.full-width {
  grid-column: 1 / -1;
}

.form-group label {
  font-size: 13px;
  font-weight: 600;
  color: #374151;
  text-transform: capitalize;
}

.required {
  color: #dc2626;
  font-weight: 700;
  margin-left: 0.2rem;
}

/* ================================================================
   FORM FIELD STYLING - Enhanced focus & hover states
   ================================================================ */
:deep(.v-field__input) {
  padding-left: 0.75rem !important;
  padding-right: 0.75rem !important;
  font-size: 14px;
}

:deep(.v-field__input input),
:deep(.v-field__input textarea) {
  padding-left: 0.5rem !important;
  padding-right: 0.5rem !important;
}

:deep(.v-field) {
  --v-field-border-opacity: 1;
}

/* Hover state - subtle border highlight */
:deep(.v-field:hover) {
  border-color: #bfdbfe !important;
}

/* Focus state - strong visual feedback */
:deep(.v-field.v-field--focused) {
  border-color: #3b82f6 !important;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1) !important;
}

/* Readonly state - visual differentiation */
:deep(.v-field[readonly]) {
  background: #f3f4f6 !important;
  opacity: 0.8;
}

:deep(.v-field[readonly] input) {
  cursor: not-allowed !important;
}

/* ================================================================
   DATETIME PICKER
   ================================================================ */
.datetime-picker-wrapper {
  display: flex;
  gap: 0.75rem;
  align-items: center;
}

.datetime-field {
  flex: 1;
}

.calendar-btn {
  height: 40px !important;
  width: 40px !important;
  min-width: 40px !important;
  color: #6b7280;
  transition: all 0.2s ease;
}

.calendar-btn:hover {
  background: #f3f4f6 !important;
  color: #1e40af;
}

.datetime-picker-card {
  border-radius: 12px;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.15) !important;
}

.picker-header {
  display: flex !important;
  align-items: center;
  padding: 1.5rem !important;
  background: linear-gradient(135deg, #1e40af 0%, #2563eb 100%) !important;
  color: white;
  border-radius: 12px 12px 0 0;
}

.picker-content {
  padding: 1.5rem;
  background: linear-gradient(135deg, #f9fbfd 0%, #f5f8fc 100%);
}

.picker-section {
  margin-bottom: 1.5rem;
}

.picker-section:last-child {
  margin-bottom: 0;
}

.picker-section label {
  display: block;
  margin-bottom: 0.75rem;
  font-size: 12px;
  font-weight: 700;
  color: #374151;
  letter-spacing: 0.3px;
}

.date-picker {
  width: 100%;
}

.hour-picker {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 1.5rem;
  padding: 1.25rem;
  background: linear-gradient(135deg, #eff6ff 0%, #e0f2fe 100%);
  border-radius: 8px;
  border: 1px solid #cce9ff;
}

.hour-display {
  font-size: 28px;
  font-weight: 700;
  color: #1e40af;
  min-width: 80px;
  text-align: center;
  font-family: 'Monaco', 'Courier New', monospace;
  letter-spacing: 2px;
}

.quick-hours {
  display: grid;
  grid-template-columns: repeat(5, 1fr);
  gap: 0.5rem;
  margin-top: 1rem;
}

.hour-btn {
  font-size: 12px !important;
  padding: 0.6rem !important;
  font-weight: 600;
  transition: all 0.2s ease;
}

.picker-footer {
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
  padding: 1.25rem !important;
  border-top: 1px solid #e0e7f1;
  background: linear-gradient(135deg, #f5f7fa 0%, #f1f3f8 100%);
  border-radius: 0 0 12px 12px;
}

/* ================================================================
   IMAGE UPLOAD SECTION
   ================================================================ */
.image-upload-section {
  display: flex;
  justify-content: center;
  margin-top: 0.5rem;
}

.image-preview {
  position: relative;
  width: 100%;
  max-width: 320px;
  height: 220px;
  border-radius: 12px;
  overflow: hidden;
  border: 2px solid #dce4f0;
  background: linear-gradient(135deg, #f9fbfd 0%, #f5f8fc 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 2px 8px rgba(30, 64, 175, 0.08);
}

.preview-img {
  width: 100%;
  height: 100%;
  object-fit: contain;
  padding: 0.5rem;
}

.remove-btn {
  position: absolute;
  top: 0.75rem;
  right: 0.75rem;
  background: rgba(0, 0, 0, 0.7);
  color: white;
  padding: 0.4rem;
  border-radius: 6px;
  transition: background 0.2s ease;
}

.remove-btn:hover {
  background: rgba(0, 0, 0, 0.9);
}

/* Interactive upload area with hover feedback */
.upload-area {
  width: 100%;
  max-width: 320px;
  height: 220px;
  border: 2px dashed #d1d5db;
  border-radius: 12px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.3s ease;
  background: linear-gradient(135deg, #f8fafd 0%, #f3f7fc 100%);
}

.upload-area:hover {
  border-color: #3b82f6;
  background: #eff6ff;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.15);
}

.upload-icon {
  font-size: 40px;
  color: #3b82f6;
  margin-bottom: 0.75rem;
  opacity: 0.9;
  transition: transform 0.2s ease;
}

.upload-area:hover .upload-icon {
  transform: scale(1.1);
}

.upload-text {
  margin: 0;
  font-size: 13px;
  font-weight: 600;
  color: #374151;
  text-align: center;
}

.upload-hint {
  margin: 0.5rem 0 0 0;
  font-size: 11px;
  color: #9ca3af;
  text-align: center;
}

/* ================================================================
   SUMMARY SECTION
   ================================================================ */
.summary-section {
  background: linear-gradient(135deg, #f9fbfd 0%, #f5f8fc 100%);
  padding: 1.5rem;
  border-radius: 8px;
  border: 1px solid #e0e7f1;
  box-shadow: 0 2px 8px rgba(30, 64, 175, 0.08);
}

.summary-card {
  background: linear-gradient(135deg, #f5f7fa 0%, #f1f3f8 100%);
  border-radius: 8px;
  padding: 1.25rem;
  margin-bottom: 1.5rem;
  border: 1px solid #dce1e8;
}

.summary-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem 0;
  font-size: 13px;
}

.summary-row + .summary-row {
  border-top: 1px solid #e5e7eb;
}

.summary-row label {
  color: #6b7280;
  font-weight: 500;
}

.summary-row span {
  color: #1f2937;
  font-weight: 600;
  font-family: 'Monaco', 'Courier New', monospace;
  letter-spacing: 0.3px;
}

/* Final total row - strong visual emphasis */
.summary-row.final {
  padding-top: 1rem;
  margin-top: 1rem;
  border-top: 2px solid #e5e7eb;
  font-size: 14px;
}

.summary-row.final label {
  color: #374151;
  font-weight: 700;
}

.summary-row.final .amount {
  color: #059669;
  font-weight: 700;
  font-size: 18px;
}

/* ================================================================
   PAYMENT DETAILS SECTION
   ================================================================ */
.payment-details-section {
  background: linear-gradient(135deg, #f0fdf4 0%, #ecfdf5 100%);
  border-radius: 8px;
  padding: 1.5rem;
  border: 1px solid #a7f3d0;
  box-shadow: 0 2px 8px rgba(5, 150, 105, 0.1);
}

.payment-details-section .section-title {
  color: #059669;
  margin-top: 0;
}

/* ================================================================
   FOOTER WITH ACTION BUTTONS
   ================================================================ */
.detail-footer {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1rem;
  padding: 1.5rem;
  border-top: 1px solid #e0e7f1;
  background: linear-gradient(135deg, #f9fbfd 0%, #f5f8fc 100%);
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(30, 64, 175, 0.08);
}

.detail-footer :deep(.v-btn) {
  padding: 0.65rem 1.5rem !important;
  text-transform: none;
  font-weight: 600;
  letter-spacing: 0.3px;
  font-size: 14px;
  border-radius: 6px;
  transition: all 0.2s ease;
}

.detail-footer :deep(.v-btn:hover) {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.12) !important;
}

/* ================================================================
   LOADING STATE
   ================================================================ */
.loading-container {
  height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 1rem;
  color: #6b7280;
}

/* ================================================================
   TEXTAREA & SELECT STYLING
   ================================================================ */
:deep(.v-textarea__control textarea) {
  padding-left: 0.75rem !important;
  padding-right: 0.75rem !important;
  font-size: 14px;
}

:deep(.v-select__content) {
  padding-left: 0.75rem !important;
  padding-right: 0.75rem !important;
}

:deep(.v-btn__content) {
  padding-left: 0.5rem !important;
  padding-right: 0.5rem !important;
  gap: 0.5rem;
}

:deep(.v-btn) {
  padding: 0 1rem !important;
}

:deep(.v-btn--size-small) {
  padding: 0 0.75rem !important;
}

:deep(.v-btn--size-x-small) {
  padding: 0 0.5rem !important;
}

:deep(.v-date-picker) {
  padding: 0.5rem;
}

.form-group :deep(.v-field) {
  margin-top: 0;
}

/* ================================================================
   RESPONSIVE DESIGN
   ================================================================ */

@media (max-width: 1024px) {
  .detail-container {
    padding: 1.5rem 1rem;
  }

  .form-grid {
    grid-template-columns: 1fr;
  }

  .info-grid {
    grid-template-columns: 1fr;
  }

  .quick-hours {
    grid-template-columns: repeat(4, 1fr);
  }

  .detail-header {
    gap: 1rem;
  }

  .detail-header h1 {
    font-size: 28px;
  }
}

@media (max-width: 768px) {
  .detail-container {
    padding: 1rem;
  }

  .detail-header {
    flex-direction: column;
    align-items: flex-start;
    margin-bottom: 1.5rem;
  }

  .header-status {
    width: 100%;
    margin-left: 0;
  }

  .detail-footer {
    flex-direction: column;
  }

  .detail-footer :deep(.v-btn) {
    width: 100%;
  }

  .quick-hours {
    grid-template-columns: repeat(3, 1fr);
  }

  .info-grid,
  .form-grid {
    gap: 1rem;
  }

  .detail-main {
    gap: 1.5rem;
  }
}

@media (max-width: 480px) {
  .detail-header h1 {
    font-size: 24px;
  }

  .quick-hours {
    grid-template-columns: repeat(2, 1fr);
  }

  .edit-section,
  .info-section,
  .summary-section,
  .payment-details-section {
    padding: 1rem;
  }
}

</style>
