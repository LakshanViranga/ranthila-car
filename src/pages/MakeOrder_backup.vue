<template>
  <div class="reservation-page">
    <!-- Header Component -->
    <header-component
        :userName="`Logged in as ${loggedUser}`"
        :showBackButton="true"
        @logout="handleLogout"
    />

    <!-- Customer Registration Modal -->
    <v-dialog
        v-model="showRegistrationModal"
        max-width="700px"
        persistent
    >
      <v-card class="registration-card">
        <!-- Modal Header -->
        <v-card-title class="modal-header">
          <span class="modal-title">Register New Customer</span>
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
                  readonly
                  class="form-input"
              />
              <p class="form-hint">This is the number you searched for</p>
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
                  class="form-input"
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

            <!-- Agreement Checkbox -->
            <div class="agreement-section">
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
              color="success"
              @click="registerNewCustomer"
              :loading="isRegistering"
              :disabled="!isRegistrationFormValid"
          >
            <v-icon icon="fa fa-user-plus" start />
            Register Customer
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
              <span class="step-number">1</span>
              <h3 class="step-title">Customer Verification</h3>
            </div>

            <div class="customer-search-section">
              <div class="customer-search-row">
                <v-text-field
                    v-model="customerIdInput"
                    label="National ID"
                    placeholder="Enter customer National Identity Card Number"
                    class="customer-id-field"
                    density="compact"
                    variant="outlined"
                    hide-details
                    :disabled="isCustomerVerified"
                    @keyup.enter="searchCustomer"
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

              <!-- Customer Info Display -->
              <div v-if="customerName" class="customer-info">
                <div class="customer-detail">
                  <p class="detail-label">Name:</p>
                  <p class="detail-value">{{ customerName }}</p>
                </div>
                <div class="customer-detail">
                  <p class="detail-label">Status:</p>
                  <p
                      class="detail-value"
                      :class="{ 'status-blacklisted': isBlacklisted, 'status-verified': !isBlacklisted }"
                  >
                    {{ isBlacklisted ? '🚫 Blacklisted' : '✓ Verified' }}
                  </p>
                </div>
              </div>

              <!-- Blacklist Error -->
              <div v-if="isBlacklisted && customerName" class="blacklist-alert">
                <v-icon icon="fa fa-exclamation-circle" />
                <span>This customer is blacklisted and cannot make reservations.</span>
              </div>
            </div>
          </div>

          <!-- Step 2: Vehicle & Details Section (Only visible if customer is verified) -->
          <div v-if="isCustomerVerified && !isBlacklisted" class="step-section">
            <div class="step-header">
              <span class="step-number">2</span>
              <h3 class="step-title">Reservation Details</h3>
            </div>

            <div class="form-group">
              <label class="form-label">Select Vehicle</label>
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
              />
            </div>

            <div class="form-group">
              <label class="form-label">Starting Time</label>
              <v-text-field
                  v-model="reservation.startingTime"
                  type="datetime-local"
                  class="time-input"
                  density="compact"
                  variant="outlined"
                  hide-details
              />
            </div>

            <div class="form-group">
              <label class="form-label">Starting Mileage (km)</label>
              <v-text-field
                  v-model.number="reservation.startingMileage"
                  type="number"
                  placeholder="0"
                  class="mileage-input"
                  density="compact"
                  variant="outlined"
                  hide-details
                  min="0"
              />
            </div>
            <div class="form-group">
              <label class="form-label">Notes</label>
              <v-text-field
                  v-model="reservation.startingMileage"
                  placeholder="Add notes if available"
                  class="mileage-input"
                  density="compact"
                  variant="outlined"
                  hide-details
              />
            </div>
            <div class="form-group">
              <label class="form-label">Guarantee Property Type</label>
              <v-text-field
                  v-model="reservation.guaranteePropertyType"
                  placeholder="Enter guarantee property details"
                  class="mileage-input"
                  density="compact"
                  variant="outlined"
                  hide-details
              />
            </div>
            <div class="form-group">
              <label class="form-label">Guarantee Property Name</label>
              <v-text-field
                  v-model="reservation.guaranteeProperty"
                  placeholder="Enter guarantee property details"
                  class="mileage-input"
                  density="compact"
                  variant="outlined"
                  hide-details
              />
            </div>

            <div class="form-group">
              <label class="form-label">License Front Image <span class="required">*</span></label>
              <div class="image-upload-section">
                <div class="image-preview" v-if="newCustomerForm.licenseFrontImage">
                  <img :src="newCustomerForm.licenseFrontImage" alt="License Front" class="preview-img" />
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
                      @change="(e) => handleImageUpload(e, 'licenseFrontImage')"
                  />
                  <v-icon icon="fa fa-camera" class="upload-icon" />
                  <p class="upload-text">Click to upload license front</p>
                  <p class="upload-hint">JPG, PNG (max 5MB)</p>
                </div>
              </div>
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
                <span class="detail-val">{{ formatPrice(selectedVehicle.dailyRate) }}</span>
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
              <span class="summary-value">{{ reservation.startingMileage }}</span>
            </div>
          </div>

          <!-- Billing Summary -->
          <div v-if="isCustomerVerified && !isBlacklisted" class="summary-card">
            <h4 class="card-title">Billing</h4>
            <div class="summary-row">
              <span class="summary-label">Daily Rate</span>
              <span class="summary-value">{{ formatPrice(selectedVehicle?.dailyRate || 0) }}</span>
            </div>
            <div class="summary-row">
              <span class="summary-label">Advanced Payment</span>
              <v-text-field
                  v-model="reservation.startingMileage"
                  placeholder="Add notes if available"
                  class="mileage-input"
                  density="compact"
                  variant="outlined"
                  hide-details
              />
            </div>
            <v-divider class="summary-divider" />

            <div class="summary-row final-row">
              <span class="final-label">Amount Due</span>

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
  </div>
</template>

<script>
import HeaderComponent from '../component/Header.vue';
import { dbService } from '../services/db.ts';
import { useProductStore } from "../stores/product.ts";

export default {
  name: 'VehicleReservation',
  components: {
    HeaderComponent
  },
  data() {
    return {
      loggedUser: 'Admin User',
      reservationId: null,

      // Customer Search
      customerIdInput: '',
      customerName: '',
      isSearching: false,
      isCustomerVerified: false,
      isBlacklisted: false,

      // Registration Modal
      showRegistrationModal: false,
      isRegistering: false,
      agreeToTerms: false,
      newCustomerForm: {
        name: '',
        identityNumber: '',
        licenseNumber: '',
        contactNumber: '',
        address: '',
        licenseFrontImage: null,
        licenseBackImage: null
      },
      licenseFrontImagePreview: null,
      licenseBackImagePreview: null,
      // Reservation Data
      reservation: {
        vehicleNumber: null,
        startingTime: '',
        startingMileage: 0,
      },

      // Mock vehicle data (replace with actual store/API)
      vehicles: Array.from(useProductStore().products.values()).map(i => ({
        ...i,
        name: i.manufacturer + ' - ' + i.model_name + ' - ' + i.register_number,
      })),
      selectedVehicle: null
    };
  },
  computed: {
    isFormValid() {
      return this.isCustomerVerified &&
          !this.isBlacklisted &&
          this.reservation.vehicleNumber &&
          this.reservation.startingTime &&
          this.reservation.startingMileage !== null;
    },
    isRegistrationFormValid() {
      return this.newCustomerForm.name.trim() &&
          this.newCustomerForm.identityNumber.trim() &&
          this.newCustomerForm.licenseNumber.trim() &&
          this.newCustomerForm.contactNumber.trim() &&
          this.newCustomerForm.address.trim() &&
          this.newCustomerForm.licenseFrontImage &&
          this.newCustomerForm.licenseBackImage &&
          this.agreeToTerms;
    }
  },
  methods: {
    /**
     * Search and verify customer by National ID
     */
    async searchCustomer() {
      if (!this.customerIdInput.trim()) {
        alert('Please enter a National ID');
        return;
      }

      this.isSearching = true;
      try {
        // Call backend service to fetch customer details
        const customer = await dbService.getCustomerByIdentity(this.customerIdInput);
        if (customer.length > 0) {

          this.customerName = customer[0].customer_name;
          this.isBlacklisted = customer[0].is_blacked_listed || false;

          // Mark as verified only if not blacklisted
          if (!this.isBlacklisted) {
            this.isCustomerVerified = true;
          }
        } else {
          // Customer not found - show registration modal
          this.openRegistrationModal();
        }
      } catch (error) {
        console.log('Error searching customer:', error);
        alert('Error searching for customer');
        this.customerName = '';
        this.isCustomerVerified = false;
        this.isBlacklisted = false;
      } finally {
        this.isSearching = false;
      }
    },

    /**
     * Open registration modal and pre-fill identity number
     */
    openRegistrationModal() {
      this.newCustomerForm = {
        name: '',
        identityNumber: this.customerIdInput,
        licenseNumber: '',
        contactNumber: '',
        address: '',
        licenseFrontImage: null,
        licenseBackImage: null
      };
      this.agreeToTerms = false;
      this.showRegistrationModal = true;
    },

    /**
     * Close registration modal
     */
    closeRegistrationModal() {
      this.showRegistrationModal = false;
      // Reset form fields
      this.newCustomerForm = {
        name: '',
        identityNumber: '',
        licenseNumber: '',
        contactNumber: '',
        address: '',
        licenseFrontImage: null,
        licenseBackImage: null
      };
      this.agreeToTerms = false;
    },

    /**
     * Trigger file input for image upload
     */
    triggerFileInput(refName) {
      this.$refs[refName].click();
    },

    /**
     * Handle image upload
     */
    async handleImageUpload(event, fieldName, previewFieldName) {
      const file = event.target.files[0];
      if (!file) return;
      // Validate file size (max 5MB)
      const maxSize = 5 * 1024 * 1024; // 5MB
      if (file.size > maxSize) {
        alert('Image size must be less than 5MB');
        return;
      }

      // Validate file type
      const allowedTypes = ['image/jpeg', 'image/png', 'image/jpg'];
      if (!allowedTypes.includes(file.type)) {
        alert('Please upload JPG or PNG image only');
        return;
      }

      // Convert to base64
      const reader = new FileReader();
      reader.onload = (e) => {
        const result = e.target?.result;

        if (!(result instanceof ArrayBuffer)) {
          return;
        }
        const byteArray = new Uint8Array(result);
        // If sending to Tauri invoke:
        this.newCustomerForm[fieldName] = Array.from(byteArray);

        // Create preview
        const blob = new Blob([byteArray], {
          type: file.type,
        });

        this[previewFieldName] = URL.createObjectURL(blob);
      };
      reader.readAsArrayBuffer(file);

      // Reset input
      event.target.value = '';
    },

    /**
     * Clear uploaded image
     */
    clearImage(fieldName) {
      this.newCustomerForm[fieldName] = null;
    },

    /**
     * Register new customer
     */
    async registerNewCustomer() {
      if (!this.isRegistrationFormValid) {
        alert('Please fill in all required fields and agree to terms');
        return;
      }

      this.isRegistering = true;
      try {
        const registrationData = {
          customer_id: this.newCustomerForm.identityNumber,
          customer_name: this.newCustomerForm.name,
          identity_number: this.newCustomerForm.identityNumber,
          license_number: this.newCustomerForm.licenseNumber,
          contact_no: this.newCustomerForm.contactNumber,
          address: this.newCustomerForm.address,
          license_front_image: this.newCustomerForm.licenseFrontImage,
          license_back_image: this.newCustomerForm.licenseBackImage,
          created_by: "admin"
        };

        await dbService.createCustomer(registrationData);

        // Customer registered successfully
        this.customerName = this.newCustomerForm.name;
        this.isCustomerVerified = true;
        this.isBlacklisted = false;

        // Close modal
        this.showRegistrationModal = false;

        // Show success message
        alert('Customer registered successfully!');
      } catch (error) {
        console.log('Error registering customer:', error);
        alert('Error registering customer. Please try again.');
      } finally {
        this.isRegistering = false;
      }
    },

    /**
     * Handle vehicle selection
     */
    onVehicleSelect(vehicleId) {
      this.selectedVehicle = this.vehicles.find(v => v.id === vehicleId) || null;
    },

    /**
     * Format price to LKR currency
     */
    formatPrice(value) {
      return new Intl.NumberFormat('en-US', {
        style: 'currency',
        currency: 'LKR',
        minimumFractionDigits: 2,
        maximumFractionDigits: 2
      }).format(value || 0);
    },

    /**
     * Format datetime for display
     */
    formatDateTime(dateTimeString) {
      if (!dateTimeString) return '-';
      const date = new Date(dateTimeString);
      return date.toLocaleString('en-US', {
        year: 'numeric',
        month: 'short',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit'
      });
    },

    /**
     * Handle reservation checkout
     */
    async handleCheckout() {
      if (!this.isFormValid) {
        alert('Please fill in all required fields');
        return;
      }
      const order_id = await dbService.getOrderId();
      const startTime = new Date(this.reservation.startingTime);
      startTime.setHours(startTime.getHours()+24);
      const finishTime = `${startTime.getFullYear()}-${String(startTime.getMonth() + 1).padStart(2, '0')}-${String(startTime.getDate()).padStart(2, '0')}T` +
          `${String(startTime.getHours()).padStart(2, '0')}:${String(startTime.getMinutes()).padStart(2, '0')}`;
      const reservationRequest = {
        order_number: order_id,
        customer_id: this.customerIdInput,
        vehicle_id: this.selectedVehicle.vehicle_id,
        starting_mileage: this.reservation.startingMileage,
        release_time: this.reservation.startingTime,
        handover_time: finishTime,
        payment_status: 'NOT PAID',
        order_status: "pending",
        notes: 'sample notes',
        created_by: 'admin'
      };

      try {
        console.log('Reservation Request:', reservationRequest);
        await dbService.createOrder(reservationRequest);

        alert('Reservation confirmed successfully!');
        // Navigate to reservations page
        this.$router.push('/view-order');
      } catch (error) {
        console.log('Error creating reservation:', error);
        alert('Error creating reservation. Please try again.');
      }
    },

    /**
     * Handle logout
     */
    handleLogout() {
      this.$emit('logout');
    }
  },
  async mounted() {
    const vehicleList = await dbService.getVehicles();
    this.vehicles = Array.from(vehicleList).map(i => ({
      ...i,
      name: i.manufacturer + ' - ' + i.model_name + '-[' + i.register_number + ']',
    }))
  }
};
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
.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.65rem;
}

.form-label {
  font-size: 12px;
  font-weight: 700;
  color: var(--color-text-primary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
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
