<template>
  <div class="reservation-page">
    <!-- Header Component -->
    <header-component
        :userName="`Logged in as ${loggedUser}`"
        :showBackButton="true"
        @logout="handleLogout"
    />

    <!-- Customer Registration/Edit Modal -->
    <v-dialog
        v-model="showRegistrationModal"
        max-width="700px"
        persistent
    >
      <v-card class="registration-card">
        <!-- Modal Header -->
        <v-card-title class="modal-header">
          <span class="modal-title">
            {{ isEditingCustomer ? 'View/Edit Customer Details' : 'Register New Customer' }}
          </span>
          <v-btn
              icon
              size="small"
              variant="text"
              @click="closeRegistrationModal"
              :disabled="isRegistering"
          >
            <v-icon icon="fa fa-times" />
          </v-btn>
        </v-card-title>

        <!-- Modal Content -->
        <v-card-text class="modal-content">
          <div class="registration-form">
            <!-- Customer Name -->
            <div class="form-group">
              <label class="form-label">Customer Name <span class="required">*</span></label>
              <v-text-field
                  v-model="newCustomerForm.name"
                  placeholder="Enter full name"
                  density="compact"
                  variant="outlined"
                  hide-details
                  class="form-input"
              />
            </div>

            <!-- Identity Number -->
            <div class="form-group">
              <label class="form-label">Identity Number (NIC) <span class="required">*</span></label>
              <v-text-field
                  v-model="newCustomerForm.identityNumber"
                  placeholder="Enter identity card number"
                  density="compact"
                  variant="outlined"
                  hide-details
                  :readonly="isEditingCustomer"
                  class="form-input"
              />
              <p v-if="!isEditingCustomer" class="form-hint">This is the number you searched for</p>
            </div>

            <!-- License Number -->
            <div class="form-group">
              <label class="form-label">License Number <span class="required">*</span></label>
              <v-text-field
                  v-model="newCustomerForm.licenseNumber"
                  placeholder="Enter driving license number"
                  density="compact"
                  variant="outlined"
                  hide-details
                  class="form-input"
              />
            </div>

            <!-- Contact Number -->
            <div class="form-group">
              <label class="form-label">Contact Number <span class="required">*</span></label>
              <v-text-field
                  v-model="newCustomerForm.contactNumber"
                  placeholder="Enter phone number"
                  density="compact"
                  variant="outlined"
                  hide-details
                  class="form-input"
              />
            </div>

            <!-- Address -->
            <div class="form-group">
              <label class="form-label">Address <span class="required">*</span></label>
              <v-textarea
                  v-model="newCustomerForm.address"
                  placeholder="Enter residential address"
                  density="compact"
                  variant="outlined"
                  hide-details
                  rows="3"
                  class="form-input form-address"
              />
            </div>

            <!-- License Front Image -->
            <div class="form-group">
              <label class="form-label">License Front Image <span class="required">*</span></label>
              <div class="image-upload-section">
                <div class="image-preview" v-if="licenseFrontImagePreview">
                  <img :src="licenseFrontImagePreview" alt="License Front" class="preview-img" />
                  <v-btn
                      icon
                      size="small"
                      variant="text"
                      color="error"
                      @click="clearImage('licenseFrontImage')"
                      class="remove-btn"
                  >
                    <v-icon icon="fa fa-trash" />
                  </v-btn>
                </div>
                <div v-else class="upload-area" @click="triggerFileInput('licenseFrontInput')">
                  <input
                      ref="licenseFrontInput"
                      type="file"
                      accept="image/*"
                      style="display: none"
                      @change="(e) => handleImageUpload(e, 'licenseFrontImage', 'licenseFrontImagePreview')"
                  />
                  <v-icon icon="fa fa-camera" class="upload-icon" />
                  <p class="upload-text">Click to upload license front</p>
                  <p class="upload-hint">JPG, PNG (max 5MB)</p>
                </div>
              </div>
            </div>

            <!-- License Back Image -->
            <div class="form-group">
              <label class="form-label">License Back Image <span class="required">*</span></label>
              <div class="image-upload-section">
                <div class="image-preview" v-if="licenseBackImagePreview">
                  <img :src="licenseBackImagePreview" alt="License Back" class="preview-img" />
                  <v-btn
                      icon
                      size="small"
                      variant="text"
                      color="error"
                      @click="clearImage('licenseBackImage')"
                      class="remove-btn"
                  >
                    <v-icon icon="fa fa-trash" />
                  </v-btn>
                </div>
                <div v-else class="upload-area" @click="triggerFileInput('licenseBackInput')">
                  <input
                      ref="licenseBackInput"
                      type="file"
                      accept="image/*"
                      style="display: none"
                      @change="(e) => handleImageUpload(e, 'licenseBackImage', 'licenseBackImagePreview')"
                  />
                  <v-icon icon="fa fa-camera" class="upload-icon" />
                  <p class="upload-text">Click to upload license back</p>
                  <p class="upload-hint">JPG, PNG (max 5MB)</p>
                </div>
              </div>
            </div>

            <!-- Agreement Checkbox (only for new registration) -->
            <div v-if="!isEditingCustomer" class="agreement-section">
              <div class="custom-checkbox-wrapper" @click="agreeToTerms = !agreeToTerms">
                <div class="checkbox-icon">
                  <v-icon
                      :icon="agreeToTerms ? 'fa fa-check-square' : 'fa fa-square'"
                      :class="{ 'checked': agreeToTerms }"
                  />
                </div>
                <span class="agreement-text">
                  I confirm that the above information is correct and this customer is not blacklisted
                </span>
              </div>
            </div>
          </div>
        </v-card-text>

        <!-- Modal Actions -->
        <v-card-actions class="modal-actions">
          <v-btn
              variant="outlined"
              color="secondary"
              @click="closeRegistrationModal"
              :disabled="isRegistering"
          >
            Cancel
          </v-btn>
          <v-spacer />
          <v-btn
              class="btn-register"
              :color="isEditingCustomer ? 'primary' : 'success'"
              @click="isEditingCustomer ? updateCustomer() : registerNewCustomer()"
              :loading="isRegistering"
              :disabled="!isFormValidForModal"
          >
            <v-icon :icon="isEditingCustomer ? 'fa fa-save' : 'fa fa-user-plus'" start />
            {{ isEditingCustomer ? 'Update Customer' : 'Register Customer' }}
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
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
            <label>Hour (0-23)</label>
            <div class="hour-picker">
              <v-btn
                  icon
                  size="small"
                  variant="tonal"
                  @click="startHour = startHour > 0 ? startHour - 1 : 0"
              >
                <v-icon icon="fa fa-minus" />
              </v-btn>
              <div class="hour-display">{{ String(startHour).padStart(2, '0') }}:00</div>
              <v-btn
                  icon
                  size="small"
                  variant="tonal"
                  @click="startHour = startHour < 23 ? startHour + 1 : 23"
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

    <!-- Reservation Content -->
    <div class="reservation-content">
      <div class="reservation-layout">

        <!-- LEFT: Reservation Form Section -->
        <div class="form-section">

          <!-- Step 1: Customer Search Section -->
          <div class="step-section">
            <div class="step-header">
              <span class="step-number">{{ t('makeOrder.customerSearch.sectionNumber') }}</span>
              <h3 class="step-title">{{ t('makeOrder.customerSearch.heading') }}</h3>
            </div>

            <div class="customer-search-section">
              <form>
                <div class="customer-search-row">
                  <v-text-field
                      v-model="customerIdInput"
                      :label="t('field.label.nationalId')"
                      :placeholder="t('field.placeholder.nationalId')"
                      class="customer-id-field"
                      density="compact"
                      variant="outlined"
                      :disabled="isCustomerVerified"
                      @keyup.enter="searchCustomer"
                      :rules="[validateNIC]"
                  />
                  <v-btn
                      icon
                      size="small"
                      variant="tonal"
                      color="info"
                      @click="searchCustomer"
                      :loading="isSearching"
                      :disabled="isCustomerVerified"
                  >
                    <v-icon icon="fa fa-search" />
                  </v-btn>
                </div>
              </form>
              <!-- Customer Info Display -->
              <div v-if="customerName" class="customer-info">
                <div class="customer-detail">
                  <p class="detail-label">{{ t('field.label.name') }}</p>
                  <p class="detail-value">{{ customerName }}</p>
                </div>
                <div class="customer-detail">
                  <p class="detail-label">{{ t('field.label.previousOrder') }} :</p>
                  <p class="detail-value">
                    {{ customer[0]?.previous_orders_count ?? customer[0]?.order_count ?? customer[0]?.total_orders ?? 0 }}
                  </p>
                </div>
                <div class="customer-detail">
                  <p class="detail-label">{{ t('field.label.status') }}:</p>
                  <p
                      class="detail-value"
                      :class="{ 'status-blacklisted': isBlacklisted, 'status-verified': !isBlacklisted }"
                  >
                    {{ isBlacklisted ? '🚫 Blacklisted' : '✓ Verified' }}
                  </p>
                </div>
                <div class="customer-detail customer-detail-action">
                  <v-btn
                      size="small"
                      variant="tonal"
                      color="info"
                      @click="openEditCustomerModal"
                  >
                    <v-icon icon="fa fa-eye" start />
                    {{ t('field.button.view')}}
                  </v-btn>
                </div>
              </div>

              <!-- Blacklist Error -->
              <div v-if="isBlacklisted && customerName" class="blacklist-alert">
                <v-icon icon="fa fa-exclamation-circle" />
                <span>This customer is blacklisted and cannot make reservations. Access Incident Management to remove</span>
              </div>
            </div>
          </div>

          <!-- Step 2: Vehicle & Details Section (Only visible if customer is verified) -->
          <div v-if="isCustomerVerified && !isBlacklisted" class="step-section">
            <div class="step-header">
              <span class="step-number">2</span>
              <h3 class="step-title">Reservation Details</h3>
            </div>
            <!-- Selected Vehicle Details -->
            <div v-if="selectedVehicle" class="vehicle-details">
              <h4 class="details-title">Vehicle Information</h4>
              <div class="detail-row">
                <span class="detail-key">Vehicle:</span>
                <span class="detail-val">{{ selectedVehicle.name }}</span>
              </div>
              <div class="detail-row">
                <span class="detail-key">Registration:</span>
                <span class="detail-val">{{ selectedVehicle.register_number }}</span>
              </div>
              <div class="detail-row">
                <span class="detail-key">Daily Rate:</span>
                <span class="detail-val">{{ formatPrice(selectedVehicle.base_price) }}</span>
              </div>
              <div class="detail-row">
                <span class="detail-key">Vehicle Revenue Licence Expire Date:</span>
                <span class="detail-val">{{ selectedVehicle.revenue_licence_date }}</span>
              </div>
              <div class="detail-row">
                <span class="detail-key">Vehicle Insurance Expire Date:</span>
                <span class="detail-val">{{ selectedVehicle.insurance_date }}</span>
              </div>
            </div>

            <div class="form-group">
              <label class="form-label">Select Vehicle<span class="required">*</span></label>
              <v-select
                  v-model="reservation.vehicleNumber"
                  :items="vehicles"
                  item-title="name"
                  item-value="id"
                  class="vehicle-select"
                  placeholder="Choose a vehicle"
                  density="compact"
                  variant="outlined"
                  hide-details
                  @update:modelValue="onVehicleSelect"
              >
                <template #item="{ props, item }">
                  <v-list-item v-bind="props" class="vehicle-list-item">
                    <template #prepend>
                      <v-icon
                          v-if="checkDocumentStatus(item.id) === 'soon'"
                          icon="fa fa-exclamation-triangle"
                          class="document-warning-icon"
                      />
                      <v-icon
                          v-else-if="checkDocumentStatus(item.id) === 'expired'"
                          icon="fa fa-times-circle"
                          class="document-expired-icon"
                      />
                      <v-icon
                          v-else
                          icon="fa fa-check-circle"
                          class="document-ok-icon"
                      />
                    </template>
                  </v-list-item>
                </template>
              </v-select>
            </div>

            <div class="form-group">
              <label class="form-label">Starting Time <span class="required">*</span></label>
              <div class="datetime-picker-wrapper">
                <v-text-field
                    v-model="startDateTimeDisplay"
                    label="Start Date & Time"
                    placeholder="Select date and time"
                    density="compact"
                    variant="outlined"
                    readonly
                    class="datetime-field"
                    :rules="[reservationDateValidation]"
                />
                <v-btn
                    icon
                    size="small"
                    variant="tonal"
                    color="info"
                    @click="showStartDateTimePicker = true"
                    class="calendar-btn"
                >
                  <v-icon icon="fa fa-calendar" />
                </v-btn>
              </div>
            </div>

            <div class="form-group">
              <label class="form-label">is vehicle release now ?</label>
              <div class="radio-buttons-row">
                <div class="radio-option">
                  <input
                      id="radio-yes"
                      v-model="isReleaseNow"
                      type="radio"
                      :value="true"
                      class="radio-input"
                  />
                  <label for="radio-yes" class="radio-text">Yes</label>
                </div>
                <div class="radio-option">
                  <input
                      id="radio-no"
                      v-model="isReleaseNow"
                      type="radio"
                      :value="false"
                      class="radio-input"
                  />
                  <label for="radio-no" class="radio-text">No</label>
                </div>
              </div>
            </div>
            <div v-if="isReleaseNow" class="form-group">
              <label class="form-label">Starting Mileage (km)</label>
              <v-text-field
                  v-model.number="reservation.startingMileage"
                  type="number"
                  placeholder="0"
                  class="mileage-input"
                  density="compact"
                  variant="outlined"
                  :rules="[mileageValidation, currentMileageValidation]"
                  min="0"
                  :readonly="selectedVehicle === null"
              />
            </div>
            <div class="form-group">
              <label class="form-label">Contact No <span class="required">*</span></label>
              <v-text-field
                  v-model="reservation.contactNo"
                  placeholder="Add contact no. ex: 07xxxxxxxx"
                  density="compact"
                  variant="outlined"
                  :rules="[validPhoneNumber]"
              />
            </div>
            <div class="form-group">
              <label class="form-label">Notes</label>
              <v-text-field
                  v-model="reservation.notes"
                  placeholder="Add notes if available"
                  density="compact"
                  variant="outlined"
                  hide-details
              />
            </div>
            <div v-if="isReleaseNow" class="form-group">
              <label class="form-label">Guarantee Property Type  <span v-if="conditionWarning" class="validate-form-label"> Warning: Guarantee is required when having mileage</span></label>
              <v-select
                  v-model="reservation.guaranteePropertyType"
                  :items="guaranteePropertyType"
                  item-title="title"
                  item-value="value"
                  class="payment-select"
                  density="compact"
                  variant="outlined"
                  hide-details
                  placeholder="Select payment type"
              />
            </div>
            <div  v-if="isReleaseNow" class="form-group">
              <label class="form-label">Guarantee Property Name</label>
              <v-text-field
                  v-model="reservation.guaranteePropertyName"
                  placeholder="Enter guarantee property details"
                  class="mileage-input"
                  density="compact"
                  variant="outlined"
                  hide-details
              />
            </div>

            <div v-if="isReleaseNow" class="form-group">
              <label class="form-label">Customer Image With Vehicle</label>
              <div class="image-upload-section">
                <div class="image-preview" v-if="customerImageWithVehiclePreview">
                  <img :src="customerImageWithVehiclePreview" alt="License Front" class="preview-img" />
                  <v-btn
                      icon
                      size="small"
                      variant="text"
                      color="error"
                      @click="clearImage('customerImageWithVehicle')"
                      class="remove-btn"
                  >
                    <v-icon icon="fa fa-trash" />
                  </v-btn>
                </div>
                <div v-else class="upload-area" @click="triggerFileInput('customerImageWithVehicleInput')">
                  <input
                      ref="customerImageWithVehicleInput"
                      type="file"
                      accept="image/*"
                      style="display: none"
                      @change="(e) => handleImageUpload(e, 'customerImageWithVehicle', 'customerImageWithVehiclePreview')"
                  />
                  <v-icon icon="fa fa-camera" class="upload-icon" />
                  <p class="upload-text">Click to upload customer image with vehicle</p>
                  <p class="upload-hint">JPG, PNG (max 5MB)</p>
                </div>
              </div>
            </div>

          </div>

        </div>

        <!-- RIGHT: Summary Section -->
        <div class="summary-section">
          <h2 class="section-title">Reservation Summary</h2>

          <!-- Customer Info Summary -->
          <div v-if="customerName" class="summary-card">
            <h4 class="card-title">Customer Details</h4>
            <div class="summary-row">
              <span class="summary-label">Name</span>
              <span class="summary-value">{{ customerName }}</span>
            </div>
            <div class="summary-row">
              <span class="summary-label">ID</span>
              <span class="summary-value">{{ customerIdInput }}</span>
            </div>
          </div>

          <!-- Reservation Info Summary -->
          <div v-if="isCustomerVerified && !isBlacklisted" class="summary-card">
            <h4 class="card-title">Reservation Details</h4>
            <div class="summary-row">
              <span class="summary-label">Vehicle</span>
              <span class="summary-value">{{ selectedVehicle?.name || '-' }}</span>
            </div>
            <div class="summary-row">
              <span class="summary-label">Start Date/Time</span>
              <span class="summary-value">{{ formatDateTime(reservation.startingTime) }}</span>
            </div>
            <div class="summary-row">
              <span class="summary-label">Starting Mileage</span>
              <span class="summary-value">{{ reservation.startingMileage }} km</span>
            </div>
            <div class="summary-row">
              <span class="summary-label">Notes</span>
              <span class="summary-value">{{ reservation.notes }}</span>
            </div>
          </div>

          <!-- Billing Summary -->
          <div v-if="isCustomerVerified && !isBlacklisted" class="summary-card billing-card">
            <h4 class="card-title">Billing</h4>

            <div class="summary-row">
              <span class="summary-label">Daily Rate</span>
              <span class="summary-value">{{ formatPrice(selectedVehicle?.base_price || 0) }}</span>
            </div>

            <div class="billing-grid">
              <div class="billing-field">
                <label class="form-label">Advanced Payment</label>
                <v-text-field
                    v-model.number="reservation.advancedPaymentAmount"
                    type="number"
                    min="0"
                    placeholder="0.00"
                    class="mileage-input"
                    density="compact"
                    variant="outlined"
                    hide-details
                />
              </div>

              <div class="billing-field">
                <label class="form-label">Payment Type</label>
                <v-select
                    v-model="reservation.paymentType"
                    :items="paymentTypeArray"
                    item-title="title"
                    item-value="value"
                    class="payment-select"
                    density="compact"
                    variant="outlined"
                    hide-details
                    placeholder="Select payment type"
                />
              </div>
            </div>

            <div class="summary-row">
              <span class="summary-label">Payment Method</span>
              <span class="summary-value payment-badge">
                 {{ reservation.paymentType === 'credit' ? 'Credit' : 'Cash' }}
               </span>
            </div>

            <v-divider class="summary-divider" />

            <div class="summary-row final-row">
              <span class="final-label">Amount Due</span>
              <span class="final-value">
                 {{ formatPrice(Math.max((selectedVehicle?.base_price || 0) - (reservation.advancedPaymentAmount || 0), 0)) }}
               </span>
            </div>
          </div>

          <!-- Action Buttons -->
          <div class="action-buttons">
            <v-btn
                v-if="!isCustomerVerified"
                class="btn-verify"
                size="large"
                color="info"
                disabled
            >
              Verify Customer First
            </v-btn>
            <v-btn
                v-else-if="isBlacklisted"
                class="btn-verify"
                size="large"
                color="error"
                disabled
            >
              Customer Blacklisted
            </v-btn>
            <v-btn
                v-else
                class="btn-checkout"
                prepend-icon="fa fa-check"
                size="large"
                @click="handleCheckout"
                :disabled="!isFormValid"
            >
              Confirm Reservation
            </v-btn>
          </div>
        </div>

      </div>
    </div>
    <ConfirmationModal ref="confirmDialog"/>
  </div>
</template>

<script setup>
import {ref, reactive, computed, onMounted, watch} from 'vue'
import HeaderComponent from '../component/Header.vue'
import { dbService } from '../services/db.ts'
import { paymentTypes, orderStatus, paymentStatus, guaranteePropertyType, paymentTypeArray } from '../utils/constants.ts'
import { useAuthStore } from '../stores/auth.ts'
import { useSnackbar } from '../composables/useSnackbar.js'
import { useRouter } from 'vue-router'
import {commonUtils} from "../utils/common.ts";
import ConfirmationModal from "../component/ConfirmationModal.vue";
import { useI18n } from "vue-i18n";

const { showSuccess, showError } = useSnackbar()
const authStore = useAuthStore()
const router = useRouter()
const {t, locale} = useI18n();
// ============== AUTHENTICATION ==============
const loggedUser = ref(authStore.username)
const roleName = ref(authStore.role)

// ============== CUSTOMER SEARCH ==============
const customerIdInput = ref('')
const customerName = ref('')
const isSearching = ref(false)
const isCustomerVerified = ref(false)
const isBlacklisted = ref(false)
const customer = ref([])

// ============== REGISTRATION MODAL ==============
const showRegistrationModal = ref(false)
const isRegistering = ref(false)
const isEditingCustomer = ref(false)
const agreeToTerms = ref(false)
const showStartDateTimePicker = ref(false)

// ============== DATE TIME PICKER STATE ==============
const startDatePicker = ref(new Date().toISOString().split('T')[0])
const startHour = ref(6)
const quickHours = ref([6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20])

// === WARNING MODAL
const confirmDialog = ref(null)

const newCustomerForm = reactive({
  name: '',
  identityNumber: '',
  licenseNumber: '',
  contactNumber: '',
  address: '',
  customerId: null,
})

// ============== IMAGE UPLOADS ==============
const licenseFrontImagePreview = ref(null)
const licenseBackImagePreview = ref(null)
const customerImageWithVehiclePreview = ref(null)
const licenseFrontImage = ref(null)
const licenseBackImage = ref(null)
const customerImageWithVehicle = ref([])

const licenseFrontInput = ref(null)
const licenseBackInput = ref(null)
const customerImageWithVehicleInput = ref(null)

// ============== RESERVATION DATA ==============
const reservation = reactive({
  vehicleNumber: null,
  startingTime: '',
  startingMileage: 0,
  contactNo: null,
  notes: null,
  guaranteePropertyType: null,
  guaranteePropertyName: null,
  paymentType: paymentTypes.cashPayment,
  advancedPaymentAmount: 0,
})

const vehicles = ref([])
const selectedVehicle = ref(null)
const isReleaseNow = ref(false)
// ============== COMPUTED PROPERTIES ==============
const isFormValid = computed(() => {
  if (isCustomerVerified.value && !isBlacklisted.value && reservation.vehicleNumber && reservation.startingTime && reservation.contactNo){
    return true
  }
  return !(isReleaseNow.value && reservation.startingMileage < 0);
})

const conditionWarning = computed(() => {
  if (reservation.startingMileage && reservation.guaranteePropertyType === null){
    return true
  }
})

const isFormValidForModal = computed(() => {
  if (isEditingCustomer.value) {
    // For edit mode, only require non-empty fields
    return (
        newCustomerForm.name.trim() &&
        newCustomerForm.identityNumber.trim() &&
        newCustomerForm.licenseNumber.trim() &&
        newCustomerForm.contactNumber.trim() &&
        newCustomerForm.address.trim()
    )
  } else {
    // For new registration, require all fields
    return (
        newCustomerForm.name.trim() &&
        newCustomerForm.identityNumber.trim() &&
        newCustomerForm.licenseNumber.trim() &&
        newCustomerForm.contactNumber.trim() &&
        newCustomerForm.address.trim() &&
        licenseFrontImage.value &&
        licenseBackImage.value &&
        agreeToTerms.value
    )
  }
})

const startDateTimeDisplay = computed(() => {
  try {
    if (!reservation?.startingTime) return 'Not set'
    const date = new Date(reservation.startingTime)
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

watch(() =>startDateTimeDisplay.value, (newValue)=> {
  const insuranceExpiredDate = Math.floor((new Date(selectedVehicle.value.insurance_date) - new Date(newValue))/(1000 * 60 * 60 * 24))
  const revenueExpiredDate = Math.floor((new Date(selectedVehicle.value.revenue_licence_date) - new Date(newValue))/(1000 * 60 * 60 * 24))
  if (insuranceExpiredDate < 1 || revenueExpiredDate < 1){
    confirmDialog.value.open({
      title: 'Unavailable Vehicle Selected',
      subtitle: 'This action may risk',
      message: 'Are you sure you want to select this vehicle on this day?',
      type: 'warning',
      confirmText: 'Yes',
    })
  }
})

// ============== METHODS ==============

/**
 * Search and verify customer by National ID
 */
const searchCustomer = async () => {
  if (!customerIdInput.value.trim()) {
    showError('Please enter a National ID')
    return
  }

  isSearching.value = true
  try {
    const result = await dbService.getCustomerByIdentity(customerIdInput.value)
    if (result.length > 0) {
      customer.value = result
      customerName.value = result[0].customer_name
      isBlacklisted.value = result[0].is_blacked_listed || false

      if (!isBlacklisted.value) {
        isCustomerVerified.value = true
      }
    } else {
      // Customer not found - show registration modal
      openRegistrationModal()
    }
  } catch (error) {
    console.log('Error searching customer:', error)
    showError('Error searching for customer')
    customerName.value = ''
    isCustomerVerified.value = false
    isBlacklisted.value = false
  } finally {
    isSearching.value = false
  }
}

/**
 * Open registration modal for new customer
 */
const openRegistrationModal = () => {
  isEditingCustomer.value = false
  newCustomerForm.name = ''
  newCustomerForm.identityNumber = customerIdInput.value
  newCustomerForm.licenseNumber = ''
  newCustomerForm.contactNumber = ''
  newCustomerForm.address = ''
  newCustomerForm.customerId = null
  agreeToTerms.value = false
  licenseFrontImagePreview.value = null
  licenseBackImagePreview.value = null
  licenseFrontImage.value = null
  licenseBackImage.value = null
  showRegistrationModal.value = true
}

/**
 * Open modal to edit existing customer details
 */
const openEditCustomerModal = async () => {
  if (customer.value.length === 0) {
    showError('No customer data available')
    return
  }

  isEditingCustomer.value = true
  const customerData = customer.value[0]
  console.log(customerData)
  // Pre-fill form with existing customer data
  newCustomerForm.customerId = customerData.id
  newCustomerForm.name = customerData.customer_name
  newCustomerForm.identityNumber = customerData.identity_number
  newCustomerForm.licenseNumber = customerData.license_number
  newCustomerForm.contactNumber = customerData.contact_no
  newCustomerForm.address = customerData.address

  // Load existing images if available
  if (customerData.license_front_image) {
    try {
      const blob = new Blob([new Uint8Array(customerData.license_front_image)], {
        type: 'image/jpg',
      })
      licenseFrontImagePreview.value = URL.createObjectURL(blob)
      licenseFrontImage.value = customerData.license_front_image
    } catch (e) {
      console.log('Error loading front image:', e)
    }
  }

  if (customerData.license_back_image) {
    try {
      const blob = new Blob([new Uint8Array(customerData.license_back_image)], {
        type: 'image/jpeg',
      })
      licenseBackImagePreview.value = URL.createObjectURL(blob)
      licenseBackImage.value = customerData.license_back_image
    } catch (e) {
      console.log('Error loading back image:', e)
    }
  }

  showRegistrationModal.value = true
}

/**
 * Close registration modal
 */
const closeRegistrationModal = () => {
  showRegistrationModal.value = false
  isEditingCustomer.value = false

  // Reset form
  newCustomerForm.name = ''
  newCustomerForm.identityNumber = ''
  newCustomerForm.licenseNumber = ''
  newCustomerForm.contactNumber = ''
  newCustomerForm.address = ''
  newCustomerForm.customerId = null
  agreeToTerms.value = false
  licenseFrontImagePreview.value = null
  licenseBackImagePreview.value = null
  licenseFrontImage.value = null
  licenseBackImage.value = null
}

/**
 * Trigger file input for image upload
 */
const triggerFileInput = (refName) => {
  if (refName === 'licenseFrontInput') {
    licenseFrontInput.value?.click()
  } else if (refName === 'licenseBackInput') {
    licenseBackInput.value?.click()
  } else if (refName === 'customerImageWithVehicleInput') {
    customerImageWithVehicleInput.value?.click()
  }
}

/**
 * Handle image upload
 */
const handleImageUpload = async (event, fieldName, previewFieldName) => {
  const file = event.target.files[0]
  if (!file) return

  // Validate file size (max 5MB)
  const maxSize = 5 * 1024 * 1024
  if (file.size > maxSize) {
    showError('Image size must be less than 5MB')
    return
  }

  // Validate file type
  const allowedTypes = ['image/jpeg', 'image/png', 'image/jpg']
  if (!allowedTypes.includes(file.type)) {
    showError('Please upload JPG or PNG image only')
    return
  }

  // Convert to base64
  const reader = new FileReader()
  reader.onload = (e) => {
    const result = e.target?.result

    if (!(result instanceof ArrayBuffer)) {
      return
    }

    const byteArray = new Uint8Array(result)

    // Store byte array
    if (fieldName === 'licenseFrontImage') {
      licenseFrontImage.value = Array.from(byteArray)
    } else if (fieldName === 'licenseBackImage') {
      licenseBackImage.value = Array.from(byteArray)
    } else if (fieldName === 'customerImageWithVehicle') {
      customerImageWithVehicle.value = Array.from(byteArray)
    }

    // Create preview
    const blob = new Blob([byteArray], { type: file.type })
    if (previewFieldName === 'licenseFrontImagePreview') {
      licenseFrontImagePreview.value = URL.createObjectURL(blob)
    } else if (previewFieldName === 'licenseBackImagePreview') {
      licenseBackImagePreview.value = URL.createObjectURL(blob)
    } else if (previewFieldName === 'customerImageWithVehiclePreview') {
      customerImageWithVehiclePreview.value = URL.createObjectURL(blob)
    }
  }

  reader.readAsArrayBuffer(file)
  event.target.value = ''
}

/**
 * Clear uploaded image
 */
const clearImage = (fieldName) => {
  if (fieldName === 'licenseFrontImage') {
    licenseFrontImagePreview.value = null
    licenseFrontImage.value = null
  } else if (fieldName === 'licenseBackImage') {
    licenseBackImagePreview.value = null
    licenseBackImage.value = null
  } else if (fieldName === 'customerImageWithVehicle') {
    customerImageWithVehiclePreview.value = null
    customerImageWithVehicle.value = []
  }
}

/**
 * Confirm start date and time
 */
const confirmStartDateTime = () => {
  if (!startDatePicker.value) {
    showError('Please select a date')
    return
  }
  // Format: YYYY-MM-DDTHH:00
  const dateTimeString = `${commonUtils.formatLocalDate(startDatePicker.value)}T${String(startHour.value).padStart(2, '0')}:00`
  reservation.startingTime = dateTimeString
  showStartDateTimePicker.value = false
}

/**
 * Register new customer
 */
const registerNewCustomer = async () => {
  if (!isFormValidForModal.value) {
    showError('Please fill in all required fields and agree to terms')
    return
  }

  isRegistering.value = true
  try {
    const registrationData = {
      customer_id: newCustomerForm.identityNumber,
      customer_name: newCustomerForm.name,
      identity_number: newCustomerForm.identityNumber,
      license_number: newCustomerForm.licenseNumber,
      contact_no: newCustomerForm.contactNumber,
      address: newCustomerForm.address,
      license_front_image: licenseFrontImage.value,
      license_back_image: licenseBackImage.value,
      created_by: loggedUser.value,
    }

    await dbService.createCustomer(registrationData)

    // Update local state
    customerName.value = newCustomerForm.name
    isCustomerVerified.value = true
    isBlacklisted.value = false

    closeRegistrationModal()
    showSuccess('Customer registered successfully!')
  } catch (error) {
    console.log('Error registering customer:', error)
    showError('Error registering customer. Please try again.')
  } finally {
    isRegistering.value = false
  }
}

/**
 * Update existing customer details
 */
const updateCustomer = async () => {
  if (!isFormValidForModal.value) {
    showError('Please fill in all required fields')
    return
  }

  isRegistering.value = true
  try {
    const updateData = {
      id: newCustomerForm.customerId,
      customer_name: newCustomerForm.name,
      license_number: newCustomerForm.licenseNumber,
      contact_no: newCustomerForm.contactNumber,
      address: newCustomerForm.address,
      updated_by: loggedUser.value,
    }

    // Only include images if they were changed
    if (licenseFrontImage.value && typeof licenseFrontImage.value !== 'object') {
      updateData.license_front_image = licenseFrontImage.value
    }
    if (licenseBackImage.value && typeof licenseBackImage.value !== 'object') {
      updateData.license_back_image = licenseBackImage.value
    }

    await dbService.updateCustomer(updateData)

    // Update local state
    customerName.value = newCustomerForm.name

    closeRegistrationModal()
    showSuccess('Customer updated successfully!')
  } catch (error) {
    console.log('Error updating customer:', error)
    showError('Error updating customer. Please try again.')
  } finally {
    isRegistering.value = false
  }
}

/**
 * Handle vehicle selection
 */
const onVehicleSelect = (vehicleId) => {
  selectedVehicle.value =
      vehicles.value.find((v) => v.vehicle_id === vehicleId || v.id === vehicleId) || null
}

/**
 * Vehicle document check
 */
const checkDocumentStatus = (vehicleId) => {
  const v = vehicles.value.find((v) => v.vehicle_id === vehicleId || v.id === vehicleId)
  const toInsuranceExpire = Math.abs(new Date(v.insurance_date) - new Date) / (1000 * 60 * 60 * 24)
  const toRevenueLicenseExpire = Math.abs(new Date(v.revenue_licence_date) - new Date) / (1000 * 60 * 60 * 24)
  if (toInsuranceExpire < 1 || toRevenueLicenseExpire < 1){
    return 'expired'
  } else if (toInsuranceExpire < 3 || toRevenueLicenseExpire < 3) {
    return 'soon'
  } else {
    return 'ok'
  }
}

/**
 * Format price to LKR currency
 */
const formatPrice = (value) => {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'LKR',
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(value || 0)
}

/**
 * Format datetime for display
 */
const formatDateTime = (dateTimeString) => {
  if (!dateTimeString) return '-'
  const date = new Date(dateTimeString)
  return date.toLocaleString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}

/**
 * Handle reservation checkout
 */
const handleCheckout = async () =>{
  if (reservation.startingMileage > 0) {
    confirmDialog.value.open({
      title: 'Active Order',
      subtitle: 'This action cannot be undone',
      message: 'Are you sure you want to active this order?',
      type: 'warning',
      confirmText: 'Yes',
      onConfirm: async () => {
        await savingOrder()
      },
    })
  } else {
    await savingOrder()
  }
}

const savingOrder = async () => {
  if (!isFormValid.value) {
    showError('Please fill in all required fields')
    return
  }

  try {
    const order_id = await dbService.getOrderId()

    const startTime = new Date(reservation.startingTime)
    startTime.setHours(startTime.getHours() + 24)
    const finishTime = `${startTime.getFullYear()}-${String(startTime.getMonth() + 1).padStart(2, '0')}-${String(startTime.getDate()).padStart(2, '0')}T${String(startTime.getHours()).padStart(2, '0')}:${String(startTime.getMinutes()).padStart(2, '0')}`

    const reservationRequest = {
      order_number: order_id,
      customer_id: customerIdInput.value,
      vehicle_id: selectedVehicle.value.vehicle_id,
      starting_mileage: reservation.startingMileage,
      release_time: reservation.startingTime,
      handover_time: finishTime,
      guarantee_type: reservation.guaranteePropertyType,
      guarantee_property: reservation.guaranteePropertyName,
      customer_image: customerImageWithVehicle.value,
      contact_no: reservation.contactNo,
      total_amount: selectedVehicle.value.base_price,
      advanced_payment: reservation.advancedPaymentAmount,
      payment_type: reservation.paymentType,
      payment_status:
          reservation.advancedPaymentAmount > 0
              ? paymentStatus.partialPaid
              : paymentStatus.notPaid,
      paid_amount: reservation.advancedPaymentAmount,
      order_status: isReleaseNow.value ? orderStatus.active : orderStatus.reserved,
      notes: reservation.notes,
      created_by: loggedUser.value,
    }

    console.log(reservationRequest)
    await dbService.createOrder(reservationRequest)

    if (reservationRequest.advanced_payment > 0) {
      await dbService.addTransaction({
        order_number: reservationRequest.order_number,
        vehicle_id: reservationRequest.vehicle_id,
        amount: reservationRequest.advanced_payment,
        payment_type: reservation.paymentType,
        created_by: loggedUser.value,
      })
    }

    showSuccess('Reservation confirmed successfully!')
    router.push('/view-order')
  } catch (error) {
    console.log('Error creating reservation:', error)
    showError('Error creating reservation. Please try again.')
  }
}

/**
 * Handle logout
 */
const handleLogout = () => {
  // Emit logout event if needed
}

/**
 * Field Validations
 */
const reservationDateValidation = (inputDate) => {
  const date = new Date(inputDate)
  return new Date() < date || 'Reservation date should be a future date'
}

const validateNIC = (nic) => {
  const cleanedNIC = nic.trim().toUpperCase()
  const oldNicRegex = /^\d{9}[VX]$/
  const newNicRegex = /^\d{12}$/

  if (oldNicRegex.test(cleanedNIC) || newNicRegex.test(cleanedNIC)) {
    return true
  }
  return 'Invalid NIC Number'
}

const validPhoneNumber = phoneNumber => {
  const regex = /^\+?[0-9]{10,15}$/;
  return regex.test(phoneNumber) || 'Invalid Mobile Number';
}

const mileageValidation = (input) => {
  const regex = /^\d+(\d+)?$/;
  return regex.test(input) || 'Invalid Mileage';
}

const currentMileageValidation = (input) => {
  const currentMileage = selectedVehicle?.value?.mileage;
  if (!currentMileage === null || currentMileage === undefined) {
    return true;
  }
  return currentMileage < input || `Should greater than ${currentMileage}`
}
// ============== LIFECYCLE ==============
onMounted(async () => {
  try {
    const vehicleList = await dbService.getVehicles()
    vehicles.value = Array.from(vehicleList).map((i) => ({
      ...i,
      name: `${i.manufacturer} - ${i.model_name} - [${i.register_number}]`,
    }))
    console.log(vehicles)
  } catch (error) {
    console.log('Error loading vehicles:', error)
    showError('Error loading vehicles')
  }
})

watch(()=>isReleaseNow.value, (newValue)=> {
  console.log(newValue)
})
</script>

<style scoped>
/* === GLOBAL COLOR VARIABLES === */
:root {
  --transition-fast: 0.2s ease;
  --transition-normal: 0.3s ease;
  --border-radius-lg: 12px;
}

/* === MODAL STYLES === */
.registration-card {
  border-radius: var(--border-radius-lg);
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  overflow: hidden;
}

.modal-header {
  display: flex !important;
  justify-content: space-between !important;
  align-items: center !important;
  padding: 1.75rem !important;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  gap: 1rem;
}

.modal-title {
  font-size: 19px;
  font-weight: 700;
  letter-spacing: 0.3px;
}

.modal-header :deep(.v-btn) {
  color: white !important;
}

.modal-content {
  padding: 2rem 1.75rem !important;
  max-height: 75vh;
  overflow-y: auto;
  background: var(--color-background-primary);
}

.modal-content::-webkit-scrollbar {
  width: 6px;
}

.modal-content::-webkit-scrollbar-track {
  background: transparent;
}

.modal-content::-webkit-scrollbar-thumb {
  background: var(--color-border-secondary);
  border-radius: 3px;
}

.modal-content::-webkit-scrollbar-thumb:hover {
  background: var(--color-border-primary);
}

.registration-form {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.6rem;
}

.form-label {
  font-size: 12px;
  font-weight: 700;
  color: var(--color-text-primary);
  text-transform: uppercase;
  letter-spacing: 0.6px;
}

.required {
  color: #ef4444;
  font-weight: 700;
  margin-left: 0.2rem;
}

.form-input :deep(.v-field__input) {
  font-size: 13px;
  margin-left: 10px;
}

.form-input :deep(.v-field) {
  min-height: 38px;
}

.form-hint {
  margin: 0.3rem 0 0 0;
  font-size: 11px;
  color: var(--color-text-secondary);
  font-style: italic;
  opacity: 0.8;
}

/* Warning dialog modal */
.warning-dialog {
  border-radius: 12px !important;
  overflow: hidden;
  box-shadow: 0 8px 30px rgba(0, 0, 0, 0.15) !important;
}

.warning-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 20px 20px 12px;
}

.warning-icon {
  width: 42px;
  height: 42px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  background: #fff3cd;
  color: #f59e0b;
  font-size: 20px;
}

.warning-title {
  font-size: 20px;
  font-weight: 600;
  color: #333;
}

.warning-message {
  padding: 8px 20px 20px !important;
  color: #666;
  font-size: 14px;
  line-height: 1.6;
}

.warning-actions {
  padding: 12px 20px 20px !important;
  justify-content: flex-end;
  border-top: 1px solid #f0f0f0;
}

.close-btn {
  min-width: 90px;
  border-radius: 6px;
  text-transform: none;
  font-weight: 500;
}

/* === IMAGE UPLOAD STYLES === */
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
  border-radius: 10px;
  overflow: hidden;
  border: 2px solid var(--color-border-secondary);
  background: var(--color-background-secondary);
  display: flex;
  align-items: center;
  justify-content: center;
}

.preview-img {
  width: 100%;
  height: 100%;
  object-fit: contain;
  padding: 0.5rem;
}

.remove-btn {
  position: absolute !important;
  top: 0.75rem !important;
  right: 0.75rem !important;
  background: rgba(0, 0, 0, 0.6) !important;
  color: white !important;
}

.remove-btn:hover {
  background: rgba(0, 0, 0, 0.8) !important;
}

.validate-form-label{
  color: red;
}
.upload-area {
  width: 100%;
  max-width: 320px;
  height: 220px;
  border: 2.5px dashed var(--color-border-secondary);
  border-radius: 10px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all var(--transition-normal);
  background: var(--color-background-secondary);
  position: relative;
}

.upload-area:hover {
  border-color: #667eea;
  background: rgba(102, 126, 234, 0.08);
  transform: translateY(-2px);
}

.upload-icon {
  font-size: 40px;
  color: #667eea;
  margin-bottom: 0.75rem;
  opacity: 0.8;
}

.upload-text {
  margin: 0;
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-primary);
  text-align: center;
}

.upload-hint {
  margin: 0.35rem 0 0 0;
  font-size: 11px;
  color: var(--color-text-secondary);
  text-align: center;
}

/* === AGREEMENT SECTION === */
.agreement-section {
  padding: 1.25rem;
  background: linear-gradient(135deg, rgba(245, 158, 11, 0.08) 0%, rgba(102, 126, 234, 0.05) 100%);
  border: 1px solid var(--color-border-secondary);
  border-radius: 8px;
  border-left: 4px solid #f59e0b;
}

.custom-checkbox-wrapper {
  display: flex;
  align-items: flex-start;
  gap: 0.85rem;
  cursor: pointer;
  user-select: none;
  transition: opacity var(--transition-fast);
}

.custom-checkbox-wrapper:hover {
  opacity: 0.8;
}

.checkbox-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  min-width: 24px;
  margin-top: 0.25rem;
  font-size: 20px;
  color: var(--color-border-secondary);
  transition: all var(--transition-fast);
}

.checkbox-icon .checked {
  color: #22c55e;
  font-weight: 700;
}

.custom-checkbox-wrapper:hover .checkbox-icon {
  color: #667eea;
}

.agreement-text {
  font-size: 13px;
  color: var(--color-text-primary);
  font-weight: 500;
  line-height: 1.6;
}

/* === MODAL ACTIONS === */
.modal-actions {
  padding: 1.5rem 1.75rem !important;
  border-top: 1px solid var(--color-border-tertiary);
  background: var(--color-background-secondary);
  gap: 0.75rem;
  display: flex;
  justify-content: flex-end;
}

.btn-register {
  text-transform: none !important;
  font-weight: 600 !important;
  letter-spacing: 0.3px;
  box-shadow: 0 4px 12px rgba(34, 197, 94, 0.2);
}

.btn-register:hover:not(:disabled) {
  box-shadow: 0 6px 16px rgba(34, 197, 94, 0.3);
  transform: translateY(-2px);
}

/* === CUSTOMER DETAILS MODAL === */
.customer-details-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 1.5rem;
}

.detail-card {
  display: flex;
  flex-direction: column;
  gap: 0.6rem;
  padding: 1rem;
  background: var(--color-background-secondary);
  border-radius: 8px;
  border: 1px solid var(--color-border-tertiary);
  transition: all var(--transition-fast);
}

.detail-card:hover {
  border-color: #667eea;
  box-shadow: 0 2px 8px rgba(102, 126, 234, 0.1);
}

.detail-card .detail-label {
  margin: 0;
  font-size: 10px;
  color: var(--color-text-secondary);
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.detail-card .detail-value {
  margin: 0;
  font-size: 14px;
  font-weight: 700;
  color: var(--color-text-primary);
}

.detail-card .detail-value.status-verified {
  color: #22c55e;
}

.detail-card .detail-value.status-blacklisted {
  color: #ef4444;
}

.radio-buttons-row {
  display: flex;
  gap: 20px;
}

.radio-option {
  display: flex;
  align-items: center;
  gap: 8px;
}

.radio-input {
  cursor: pointer;
}

.radio-text {
  cursor: pointer;
}

@media (max-width: 600px) {
  .customer-details-grid {
    grid-template-columns: 1fr;
  }
}

/* === DATETIME PICKER STYLES === */
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
}

.datetime-picker-card {
  border-radius: var(--border-radius-lg);
}

.picker-header {
  display: flex !important;
  align-items: center;
  padding: 1.25rem !important;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border-bottom: 1px solid var(--color-border-tertiary);
}

.picker-content {
  padding: 1.5rem;
}

.picker-section {
  margin-bottom: 1.5rem;
}

.picker-section label {
  display: block;
  margin-bottom: 0.75rem;
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-primary);
  text-transform: uppercase;
  letter-spacing: 0.3px;
}

.date-picker {
  width: 100%;
}

.hour-picker {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 1rem;
  padding: 1rem;
  background: var(--color-background-secondary);
  border-radius: 8px;
}

.hour-display {
  font-size: 24px;
  font-weight: 700;
  color: #667eea;
  min-width: 80px;
  text-align: center;
  font-family: 'Monaco', 'Courier New', monospace;
}

.quick-hours {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 0.5rem;
  margin-top: 1rem;
}

.hour-btn {
  font-size: 12px !important;
  padding: 0.5rem !important;
}

.picker-footer {
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
  padding: 1.25rem !important;
  border-top: 1px solid var(--color-border-tertiary);
  background: var(--color-background-secondary);
}

/* === MAIN PAGE LAYOUT === */
.reservation-page {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
  background: linear-gradient(135deg, var(--color-background-tertiary) 0%, var(--color-background-primary) 100%);
}

.reservation-content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.reservation-layout {
  display: flex;
  gap: 2.5rem;
  flex: 1;
  overflow: hidden;
  padding: 2.5rem;
  max-width: 1600px;
  margin: 0 auto;
  width: 100%;
}

/* === FORM SECTION === */
.form-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 1.75rem;
  overflow-y: auto;
  padding-right: 1rem;
}

.form-section::-webkit-scrollbar {
  width: 7px;
}

.form-section::-webkit-scrollbar-track {
  background: transparent;
}

.form-section::-webkit-scrollbar-thumb {
  background: var(--color-border-secondary);
  border-radius: 4px;
}

.form-section::-webkit-scrollbar-thumb:hover {
  background: var(--color-border-primary);
}

.form-address {
  margin-top: 10px;
}

/* === STEP SECTION === */
.step-section {
  background: var(--color-background-primary);
  border-radius: var(--border-radius-lg);
  border: 1px solid var(--color-border-tertiary);
  padding: 1.75rem;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
  transition: all var(--transition-normal);
}

.step-section:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
}

.step-header {
  display: flex;
  align-items: center;
  gap: 1.2rem;
  margin-bottom: 1.75rem;
  padding-bottom: 1.25rem;
  border-bottom: 2px solid var(--color-border-secondary);
}

.step-number {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 44px;
  height: 44px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border-radius: 50%;
  font-weight: 700;
  font-size: 18px;
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
  flex-shrink: 0;
}

.step-title {
  margin: 0;
  font-size: 16px;
  font-weight: 700;
  color: var(--color-text-primary);
  letter-spacing: 0.2px;
}

/* === CUSTOMER SEARCH SECTION === */
.customer-search-section {
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
}

.customer-search-row {
  display: flex;
  gap: 0.75rem;
  align-items: flex-end;
}

.customer-id-field {
  flex: 1;
  max-width: none;
}

.customer-id-field :deep(.v-field__input) {
  font-size: 13px;
}

.customer-info {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1.25rem;
  padding: 1.25rem;
  background: linear-gradient(135deg, var(--color-background-secondary) 0%, rgba(102, 126, 234, 0.04) 100%);
  border-radius: 10px;
  border-left: 4px solid #667eea;
  box-shadow: 0 2px 8px rgba(102, 126, 234, 0.1);
}

.customer-detail {
  display: flex;
  flex-direction: column;
  gap: 0.4rem;
}

.detail-label {
  margin: 0;
  font-size: 11px;
  color: var(--color-text-secondary);
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.6px;
}

.detail-value {
  margin: 0;
  font-size: 14px;
  font-weight: 700;
  color: var(--color-text-primary);
}

.detail-value.status-verified {
  color: #22c55e;
}

.detail-value.status-blacklisted {
  color: #ef4444;
}

.customer-detail-action {
  justify-content: flex-end;
  align-items: flex-start;
}

.customer-detail-action :deep(.v-btn) {
  text-transform: none;
  font-weight: 600;
}

/* === BLACKLIST ALERT === */
.blacklist-alert {
  display: flex;
  align-items: center;
  gap: 0.85rem;
  padding: 1rem 1.25rem;
  background: linear-gradient(135deg, rgba(239, 68, 68, 0.1) 0%, rgba(239, 68, 68, 0.05) 100%);
  border: 1.5px solid #ef4444;
  border-radius: 10px;
  color: #dc2626;
  font-weight: 600;
  font-size: 13px;
  animation: slideIn 0.3s ease;
}

.blacklist-alert i {
  font-size: 18px;
  flex-shrink: 0;
}

@keyframes slideIn {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* === FORM GROUPS === */
.form-label {
  font-size: 12px;
  font-weight: 700;
  color: var(--color-text-primary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin-top: 2rem;
}

.vehicle-select :deep(.v-field__input),
.time-input :deep(.v-field__input),
.mileage-input :deep(.v-field__input) {
  font-size: 13px;
}

/* === VEHICLE DETAILS === */
.vehicle-details {
  margin-top: 1.25rem;
  padding: 1.25rem;
  background: linear-gradient(135deg, var(--color-background-secondary) 0%, rgba(34, 197, 94, 0.04) 100%);
  border-radius: 10px;
  border-left: 4px solid #22c55e;
  box-shadow: 0 2px 8px rgba(34, 197, 94, 0.1);
}

.details-title {
  margin: 0 0 1rem 0;
  font-size: 12px;
  font-weight: 700;
  color: var(--color-text-primary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.detail-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.7rem 0;
  border-bottom: 1px solid var(--color-border-tertiary);
  font-size: 13px;
}

.detail-row:last-child {
  border-bottom: none;
}

.detail-key {
  color: var(--color-text-secondary);
  font-weight: 600;
}

.detail-val {
  color: var(--color-text-primary);
  font-weight: 700;
  font-family: 'Monaco', 'Courier New', monospace;
}

.document-warning-icon {
  color: var(--color-warning) !important;
  font-size: 18px !important;
}

.document-expired-icon {
  color: var(--color-error) !important;
  font-size: 18px !important;
}

.document-ok-icon {
  color: var(--color-success) !important;
  font-size: 18px !important;
}

/* === SUMMARY SECTION === */
.summary-section {
  width: 420px;
  background: var(--color-background-primary);
  border-radius: var(--border-radius-lg);
  border: 1px solid var(--color-border-tertiary);
  padding: 2rem;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  overflow-y: auto;
  position: sticky;
  top: 2.5rem;
  height: fit-content;
  max-height: calc(100vh - 5rem);
  box-shadow: 0 4px 16px rgba(0, 0, 0, 0.08);
}

.summary-section::-webkit-scrollbar {
  width: 6px;
}

.summary-section::-webkit-scrollbar-track {
  background: transparent;
}

.summary-section::-webkit-scrollbar-thumb {
  background: var(--color-border-secondary);
  border-radius: 3px;
}

.summary-section::-webkit-scrollbar-thumb:hover {
  background: var(--color-border-primary);
}

.section-title {
  margin: 0;
  font-size: 16px;
  font-weight: 700;
  color: var(--color-text-primary);
  padding-bottom: 1.25rem;
  border-bottom: 2px solid var(--color-border-secondary);
  letter-spacing: 0.2px;
}

.summary-card {
  display: flex;
  flex-direction: column;
  gap: 0.85rem;
  padding: 1.25rem;
  background: var(--color-background-secondary);
  border-radius: 10px;
  border: 1px solid var(--color-border-tertiary);
  transition: all var(--transition-fast);
}

.summary-card:hover {
  border-color: #667eea;
  box-shadow: 0 2px 8px rgba(102, 126, 234, 0.1);
}

.card-title {
  margin: 0 0 0.6rem 0;
  font-size: 11px;
  font-weight: 700;
  color: var(--color-text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.6px;
}

.summary-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  font-size: 12px;
  padding: 0.55rem 0;
  gap: 1rem;
}

.summary-label {
  color: var(--color-text-secondary);
  font-weight: 600;
  white-space: nowrap;
}

.summary-value {
  color: var(--color-text-primary);
  font-weight: 700;
  font-family: 'Monaco', 'Courier New', monospace;
  text-align: right;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.summary-divider {
  margin: 0.5rem 0 !important;
}

.final-row {
  padding: 1rem 0;
  border-top: 2px solid var(--color-border-tertiary);
  border-bottom: 2px solid var(--color-border-tertiary);
}

.final-label {
  font-size: 13px;
  font-weight: 700;
  color: var(--color-text-primary);
}

.final-value {
  font-size: 18px;
  font-weight: 700;
  color: #22c55e;
  font-family: 'Monaco', 'Courier New', monospace;
}

/* === ACTION BUTTONS === */
.action-buttons {
  display: flex;
  flex-direction: column;
  gap: 0.85rem;
  margin-top: auto;
  padding-top: 1.5rem;
  border-top: 2px solid var(--color-border-secondary);
}

.btn-verify,
.btn-checkout {
  text-transform: none !important;
  letter-spacing: 0.3px;
  font-weight: 600 !important;
  border-radius: 8px;
  padding: 0.85rem 1.25rem !important;
  transition: all var(--transition-normal);
}

.btn-checkout {
  background: linear-gradient(135deg, #22c55e 0%, #16a34a 100%) !important;
  box-shadow: 0 4px 12px rgba(34, 197, 94, 0.3);
}

.btn-checkout:hover:not(:disabled) {
  box-shadow: 0 6px 16px rgba(34, 197, 94, 0.4);
  transform: translateY(-2px);
}

.btn-checkout:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* === RESPONSIVE DESIGN === */
@media (max-width: 1400px) {
  .summary-section {
    width: 380px;
    padding: 1.5rem;
  }

  .reservation-layout {
    gap: 2rem;
    padding: 2rem;
  }
}

@media (max-width: 1200px) {
  .reservation-layout {
    flex-direction: column;
    gap: 1.5rem;
    padding: 1.5rem;
  }

  .summary-section {
    width: 100%;
    position: static;
    max-height: none;
    top: auto;
  }

  .form-section {
    padding-right: 0;
  }

  .customer-info {
    grid-template-columns: 1fr 1fr;
  }
}

@media (max-width: 768px) {
  .reservation-page {
    height: auto;
  }

  .reservation-content {
    min-height: calc(100vh - 80px);
  }

  .reservation-layout {
    flex-direction: column;
    gap: 1rem;
    padding: 1rem;
  }

  .customer-info {
    grid-template-columns: 1fr;
  }

  .step-section {
    padding: 1.25rem;
  }

  .step-header {
    gap: 0.85rem;
    margin-bottom: 1.25rem;
  }

  .step-number {
    width: 40px;
    height: 40px;
    font-size: 16px;
  }

  .step-title {
    font-size: 15px;
  }

  .customer-search-row {
    flex-direction: column;
    gap: 0.5rem;
  }

  .customer-id-field {
    max-width: 100%;
  }

  .detail-row {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.3rem;
  }

  .summary-section {
    gap: 1rem;
    padding: 1.5rem;
  }

  .action-buttons {
    gap: 0.65rem;
  }

  .form-label {
    font-size: 11px;
  }

  .modal-header {
    padding: 1.25rem !important;
  }

  .modal-title {
    font-size: 17px;
  }

  .modal-content {
    padding: 1.5rem 1.25rem !important;
  }
}

@media (max-width: 480px) {
  .reservation-layout {
    padding: 0.75rem;
  }

  .step-header {
    gap: 0.65rem;
    margin-bottom: 1rem;
  }

  .step-number {
    width: 36px;
    height: 36px;
    font-size: 14px;
  }

  .step-title {
    font-size: 14px;
  }

  .section-title {
    font-size: 14px;
  }

  .summary-row {
    font-size: 11px;
  }

  .card-title {
    font-size: 10px;
  }

  .customer-info {
    padding: 1rem;
    gap: 1rem;
  }

  .detail-label {
    font-size: 10px;
  }

  .detail-value {
    font-size: 12px;
  }

  .upload-icon {
    font-size: 32px;
  }

  .upload-text {
    font-size: 12px;
  }

  .agreement-section {
    padding: 1rem;
  }

  .agreement-text {
    font-size: 12px;
  }
}
</style>
