<template>
  <div class="vehicles-page">
    <!-- Header Component -->
    <header-component
        :userName="`Logged in as ${loggedUser}`"
        :showBackButton="true"
        @logout="handleLogout"
    />

    <!-- Vehicles Content -->
    <div class="vehicles-content">
      <div class="vehicles-container">

        <!-- Page Header with Add Button and Reminders Button -->
        <div class="page-header">
          <h1>Manage Vehicles</h1>
          <div class="header-actions">
            <button class="btn-reminders" @click="openRemindersModal" v-if="upcomingExpirations.length > 0">
              <i class="fa fa-bell"></i>
              <span>Upcoming Expirations</span>
              <span class="reminder-badge">{{ upcomingExpirations.length }}</span>
            </button>
            <button class="btn-add-vehicle" @click="openAddModal">
              <i class="fa fa-plus"></i>
              <span>Add Vehicle</span>
            </button>
          </div>
        </div>

        <!-- Vehicles Table -->
        <div class="table-section">
          <div class="table-wrapper">
            <table class="vehicles-table">
              <thead>
              <tr>
                <th>Vehicle Name</th>
                <th>Registration Number</th>
                <th>Transmission</th>
                <th>Base Price - Per Day</th>
                <th>Unit Price - Per Km</th>
                <th>Hourly Rate</th>
                <th>Distance - Per Day</th>
                <th>Actions</th>
              </tr>
              </thead>
              <tbody>
              <tr v-for="vehicle in vehicles" :key="vehicle.id" class="vehicle-row">
                <td class="vehicle-name">
                  <strong>{{ vehicle.manufacturer }} {{ vehicle.modelName }}</strong>
                </td>
                <td class="registration">
                  {{ vehicle.registerNumber }}
                </td>
                <td class="transmission">
                  <span :class="['transmission-badge', vehicle.transmissionType.toLowerCase()]">
                    {{ vehicle.transmissionType }}
                  </span>
                </td>
                <td class="price">
                  {{ formatPrice(vehicle.basePrice) }}
                </td>
                <td class="price">
                  {{ formatPrice(vehicle.unitPrice) }}
                </td>
                <td class="price">
                  {{ formatPrice(vehicle.additionalHourPrice) }}
                </td>
                <td class="price">
                  {{ vehicle.distanceRange }} km
                </td>
                <td class="actions">
                  <button
                      class="btn-action view"
                      @click="openViewModal(vehicle)"
                      title="View Details"
                  >
                    <i class="fa fa-eye"></i>
                  </button>
                  <button
                      class="btn-action edit"
                      @click="openEditModal(vehicle)"
                      title="Edit"
                  >
                    <i class="fa fa-edit"></i>
                  </button>
                  <button
                      v-if="roleName === roleTypes.admin"
                      class="btn-action delete"
                      @click="deleteVehicle(vehicle.vehicleId)"
                      title="Delete"
                  >
                    <i class="fa fa-trash"></i>
                  </button>
                </td>
              </tr>
              </tbody>
            </table>

            <!-- Empty State -->
            <div v-if="vehicles.length === 0" class="empty-state">
              <i class="fa fa-car"></i>
              <p>No vehicles found</p>
              <button class="btn-add-empty" @click="openAddModal">Add your first vehicle</button>
            </div>
          </div>
        </div>

        <!-- Summary Stats -->
        <div class="summary-stats">
          <div class="stat-card">
            <label>Total Vehicles</label>
            <span class="stat-value">{{ vehicles.length }}</span>
          </div>
          <div class="stat-card">
            <label>Manual Transmission</label>
            <span class="stat-value manual">{{ manualCount }}</span>
          </div>
          <div class="stat-card">
            <label>Automatic Transmission</label>
            <span class="stat-value automatic">{{ automaticCount }}</span>
          </div>
          <div class="stat-card">
            <label>Average Base Price</label>
            <span class="stat-value">{{ formatPrice(averageBasePrice) }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- View Details Modal -->
    <div v-if="showViewModal" class="modal-overlay" @click="closeViewModal">
      <div class="modal-content view-modal" @click.stop>
        <div class="modal-header">
          <h2>Vehicle Details</h2>
          <button class="btn-close" @click="closeViewModal">
            <i class="fa fa-times"></i>
          </button>
        </div>

        <div class="modal-body view-body">
          <div v-if="selectedVehicle" class="details-grid">
            <!-- Left Column -->
            <div class="details-column">
              <div class="detail-item">
                <label>Manufacturer</label>
                <p>{{ selectedVehicle.manufacturer }}</p>
              </div>
              <div class="detail-item">
                <label>Model Name</label>
                <p>{{ selectedVehicle.modelName }}</p>
              </div>
              <div class="detail-item">
                <label>Registration Number</label>
                <p>{{ selectedVehicle.registerNumber }}</p>
              </div>
              <div class="detail-item">
                <label>Owner</label>
                <p>{{ selectedVehicle.owner }}</p>
              </div>
              <div class="detail-item">
                <label>Fuel Type</label>
                <p>{{ selectedVehicle.fuelType }}</p>
              </div>
              <div class="detail-item">
                <label>Revenue Licence Valid Date</label>
                <p class="price-value">{{ selectedVehicle.revenueLicenseDate }}</p>
              </div>
            </div>

            <!-- Right Column -->
            <div class="details-column">
              <div class="detail-item">
                <label>Transmission Type</label>
                <p>{{ selectedVehicle.transmissionType }}</p>
              </div>
              <div class="detail-item">
                <label>Mileage</label>
                <p>{{ selectedVehicle.mileage }} km</p>
              </div>
              <div class="detail-item">
                <label>Base Price - Per Day</label>
                <p class="price-value">{{ formatPrice(selectedVehicle.basePrice) }}</p>
              </div>
              <div class="detail-item">
                <label>Unit Price (Per Km)</label>
                <p class="price-value">{{ formatPrice(selectedVehicle.unitPrice) }}</p>
              </div>
              <div class="detail-item">
                <label>Additional Hour Price</label>
                <p class="price-value">{{ formatPrice(selectedVehicle.additionalHourPrice) }}</p>
              </div>
              <div class="detail-item">
                <label>Distance Range Per Day</label>
                <p class="price-value">{{ selectedVehicle.distanceRange }}</p>
              </div>

              <div class="detail-item">
                <label>Vehicle Insurance Valid Date</label>
                <p class="price-value">{{ selectedVehicle.insuranceDate }}</p>
              </div>
            </div>
          </div>

          <div class="view-actions">
            <button class="btn-secondary" @click="closeViewModal">Close</button>
            <button class="btn-primary" @click="openEditModal(selectedVehicle)">Edit Vehicle</button>
          </div>
        </div>
      </div>
    </div>

    <!-- Upcoming Reminders Modal -->
    <div v-if="showRemindersModal" class="modal-overlay" @click="closeRemindersModal">
      <div class="modal-content reminders-modal" @click.stop>
        <div class="modal-header reminders-header">
          <h2><i class="fa fa-bell"></i> Upcoming Expirations</h2>
          <button class="btn-close" @click="closeRemindersModal">
            <i class="fa fa-times"></i>
          </button>
        </div>

        <div class="modal-body reminders-body">
          <div class="reminders-content">
            <div v-for="item in upcomingExpirations" :key="item.vehicle.id" class="reminder-card">
              <div class="reminder-vehicle-info">
                <div class="vehicle-name-section">
                  <strong>{{ item.vehicle.manufacturer }} {{ item.vehicle.modelName }}</strong>
                  <span class="reg-number">{{ item.vehicle.registerNumber }}</span>
                </div>
              </div>

              <div class="reminder-items">
                <!-- Revenue License Expiration -->
                <div v-if="item.revenueLicenseExpiring" class="expiration-item revenue">
                  <i class="fa fa-calendar"></i>
                  <div class="expiration-details">
                    <label>Revenue License Expiring</label>
                    <p>{{ item.vehicle.revenueLicenseDate }}</p>
                    <span class="days-left">{{ item.revenueLicenseDaysLeft }} days left</span>
                  </div>
                </div>

                <!-- Insurance Expiration -->
                <div v-if="item.insuranceExpiring" class="expiration-item insurance">
                  <i class="fa fa-shield"></i>
                  <div class="expiration-details">
                    <label>Insurance Expiring</label>
                    <p>{{ item.vehicle.insuranceDate }}</p>
                    <span class="days-left">{{ item.insuranceDaysLeft }} days left</span>
                  </div>
                </div>
              </div>

              <button class="btn-view-details" @click="selectVehicleAndCloseReminders(item.vehicle)">
                View Details
              </button>
            </div>
          </div>
        </div>

        <div class="reminders-footer">
          <button class="btn-secondary" @click="closeRemindersModal">Close</button>
        </div>
      </div>
    </div>

    <!-- Add/Edit Vehicle Modal -->
    <div v-if="showModal" class="modal-overlay" @click="closeModal">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h2>{{ isEditMode ? 'Edit Vehicle' : 'Add Vehicle' }}</h2>
          <button class="btn-close" @click="closeModal">
            <i class="fa fa-times"></i>
          </button>
        </div>

        <div class="modal-body">
          <form @submit.prevent="saveVehicle">
            <!-- Manufacturer -->
            <div class="form-group">
              <label for="manufacturer">Manufacturer <span class="required">*</span></label>
              <select
                  id="manufacturer"
                  v-model="formData.manufacturer"
                  class="input-field"
                  required
              >
                <option value="">Select manufacturer</option>
                <option value="Toyota">Toyota</option>
                <option value="Honda">Honda</option>
                <option value="Suzuki">Suzuki</option>
                <option value="Nissan">Nissan</option>
                <option value="Mazda">Mazda</option>
                <option value="Perodua">Perodua</option>
                <option value="Hyundai">Hyundai</option>
                <option value="Kia">Kia</option>
              </select>
            </div>

            <!-- Model Name -->
            <div class="form-group">
              <label for="modelName">Model Name <span class="required">*</span></label>
              <input
                  id="modelName"
                  v-model="formData.modelName"
                  type="text"
                  class="input-field"
                  placeholder="e.g., Prius, Civic, Swift"
                  required
                  :readonly="isEditMode"
              />
            </div>

            <!-- Registration Number -->
            <div class="form-group">
              <label for="registerNumber">Registration Number <span class="required">*</span></label>
              <v-text-field
                  id="registerNumber"
                  v-model="formData.registerNumber"
                  type="text"
                  class="input-field"
                  placeholder="e.g., SRB-1234"
                  :rules="[registerNumberValidation]"
                  required
                  :readonly="isEditMode"
              />
            </div>

            <!-- Owner -->
            <div class="form-group">
              <label for="owner">Owner <span class="required">*</span></label>
              <input
                  id="owner"
                  v-model="formData.owner"
                  type="text"
                  class="input-field"
                  placeholder="Owner name"
                  required
              />
            </div>

            <!-- Fuel Type -->
            <div class="form-group">
              <label for="fuelType">Fuel Type <span class="required">*</span></label>
              <select
                  id="fuelType"
                  v-model="formData.fuelType"
                  class="input-field"
                  required
                  :disabled="isEditMode"
              >
                <option value="">Select fuel type</option>
                <option value="Hybrid">Hybrid</option>
                <option value="Petrol">Petrol</option>
                <option value="Diesel">Diesel</option>
                <option value="Electric">Electric</option>
                <option value="LPG">LPG</option>
              </select>
            </div>

            <!-- Transmission Type -->
            <div class="form-group">
              <label for="transmissionType">Transmission Type <span class="required">*</span></label>
              <select
                  id="transmissionType"
                  v-model="formData.transmissionType"
                  class="input-field"
                  required
                  :disabled="isEditMode"
              >
                <option value="">Select transmission</option>
                <option :value="transmissionType.manual" >Manual</option>
                <option :value="transmissionType.automatic">Automatic</option>
              </select>
            </div>

            <!-- Mileage -->
            <div class="form-group">
              <label for="mileage">Mileage (km) <span class="required">*</span></label>
              <input
                  id="mileage"
                  v-model.number="formData.mileage"
                  type="number"
                  class="input-field"
                  placeholder="0"
                  min="0"
                  required
                  :readonly="isEditMode"
              />
            </div>

            <!-- Base Price -->
            <div class="form-group">
              <label for="basePrice">Base Price - Per Day (LKR) <span class="required">*</span></label>
              <input
                  id="basePrice"
                  v-model.number="formData.basePrice"
                  type="number"
                  class="input-field"
                  placeholder="0.00"
                  step="500"
                  min="0"
                  required
              />
            </div>

            <!-- Unit Price (Per Day) -->
            <div class="form-group">
              <label for="unitPrice">Unit Price - Per Km (LKR) <span class="required">*</span></label>
              <input
                  id="unitPrice"
                  v-model.number="formData.unitPrice"
                  type="number"
                  class="input-field"
                  placeholder="0.00"
                  step="5"
                  min="0"
                  required
              />
            </div>

            <!-- Additional Hour Price -->
            <div class="form-group">
              <label for="additionalHourPrice">Additional Hour Price (LKR) <span class="required">*</span></label>
              <input
                  id="additionalHourPrice"
                  v-model.number="formData.additionalHourPrice"
                  type="number"
                  class="input-field"
                  placeholder="0.00"
                  step="50"
                  min="0"
                  required
              />
            </div>

            <div class="form-group">
              <label for="additionalHourPrice">Distance Range Per Day (Km) <span class="required">*</span></label>
              <input
                  id="additionalHourPrice"
                  v-model.number="formData.distanceRange"
                  type="number"
                  class="input-field"
                  placeholder="0.00"
                  step="50"
                  min="0"
                  required
              />
            </div>
            <div class="form-group">
              <label for="expenseDate">Revenue Licence Valid Date <span class="required">*</span></label>
              <input
                  id="expenseDate"
                  v-model="formData.revenueLicenseDate"
                  type="date"
                  class="input-field"
                  required
              />
            </div>
            <div class="form-group">
              <label for="expenseDate">Vehicle Insurance Valid Date <span class="required">*</span></label>
              <input
                  id="expenseDate"
                  v-model="formData.insuranceDate"
                  type="date"
                  class="input-field"
                  required
              />
            </div>

            <!-- Form Actions -->
            <div class="form-actions">
              <button type="button" class="btn-secondary" @click="closeModal">Cancel</button>
              <button type="submit" class="btn-primary">
                {{ isEditMode ? 'Update Vehicle' : 'Add Vehicle' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>

    <ConfirmationModal ref="confirmDialog"/>
  </div>
</template>

<script>
import HeaderComponent from '../component/Header.vue';
import { dbService } from '../services/db.ts';
import {roleTypes, transmissionType} from '../utils/constants.ts'
import { useAuthStore } from "../stores/auth.ts";
import {useSnackbar} from "../composables/useSnackbar.js";
import ConfirmationModal from "../component/ConfirmationModal.vue";
import {ref} from "vue";
const { showSuccess, showError } = useSnackbar();

export default {
  name: 'VehicleManagement',
  components: {
    ConfirmationModal,
    HeaderComponent
  },
  data() {
    return {
      loggedUser: null,
      roleName: null,
      showModal: false,
      showViewModal: false,
      showRemindersModal: false,
      isEditMode: false,
      editingVehicleId: null,
      selectedVehicle: null,
      formData: {
        manufacturer: '',
        modelName: '',
        registerNumber: '',
        owner: '',
        fuelType: '',
        transmissionType: '',
        mileage: 0,
        basePrice: 0,
        unitPrice: 0,
        additionalHourPrice: 0,
        distanceRange: 0,
        revenueLicenseDate: '',
        insuranceDate: '',
      },
      vehicles: [],
      confirmDialog: ref(null),
      expirationThresholdDays: 30, // Show reminders for expirations within 30 days
    };
  },
  created() {
    const authStore = useAuthStore()
    this.loggedUser = authStore.username
    this.roleName = authStore.role
  },
  computed: {
    roleTypes() {
      return roleTypes
    },
    transmissionType() {
      return transmissionType
    },
    manualCount() {
      return this.vehicles.filter(v => v.transmissionType === transmissionType.manual).length;
    },
    automaticCount() {
      return this.vehicles.filter(v => v.transmissionType === transmissionType.automatic).length;
    },
    averageBasePrice() {
      if (this.vehicles.length === 0) return 0;
      const total = this.vehicles.reduce((sum, v) => sum + v.basePrice, 0);
      return total / this.vehicles.length;
    },
    upcomingExpirations() {
      const today = new Date();
      today.setHours(0, 0, 0, 0);
      const thresholdDate = new Date(today);
      thresholdDate.setDate(thresholdDate.getDate() + this.expirationThresholdDays);

      const expiringVehicles = [];

      this.vehicles.forEach(vehicle => {
        const revenueLicenseDate = new Date(vehicle.revenueLicenseDate);
        const insuranceDate = new Date(vehicle.insuranceDate);

        revenueLicenseDate.setHours(0, 0, 0, 0);
        insuranceDate.setHours(0, 0, 0, 0);

        const revenueLicenseExpiring = revenueLicenseDate >= today && revenueLicenseDate <= thresholdDate;
        const insuranceExpiring = insuranceDate >= today && insuranceDate <= thresholdDate;

        if (revenueLicenseExpiring || insuranceExpiring) {
          const revenueLicenseDaysLeft = this.calculateDaysLeft(revenueLicenseDate, today);
          const insuranceDaysLeft = this.calculateDaysLeft(insuranceDate, today);

          expiringVehicles.push({
            vehicle,
            revenueLicenseExpiring,
            insuranceExpiring,
            revenueLicenseDaysLeft,
            insuranceDaysLeft,
          });
        }
      });

      // Sort by days left (earliest first)
      return expiringVehicles.sort((a, b) => {
        const aDaysLeft = a.revenueLicenseExpiring ? a.revenueLicenseDaysLeft : a.insuranceDaysLeft;
        const bDaysLeft = b.revenueLicenseExpiring ? b.revenueLicenseDaysLeft : b.insuranceDaysLeft;
        return aDaysLeft - bDaysLeft;
      });
    }
  },
  methods: {
    calculateDaysLeft(expirationDate, today) {
      const timeDiff = expirationDate - today;
      const daysLeft = Math.ceil(timeDiff / (1000 * 60 * 60 * 24));
      return daysLeft;
    },

    formatPrice(value) {
      return new Intl.NumberFormat('en-US', {
        style: 'currency',
        currency: 'LKR',
        minimumFractionDigits: 2,
        maximumFractionDigits: 2
      }).format(value || 0);
    },

    openAddModal() {
      this.isEditMode = false;
      this.editingVehicleId = null;
      this.resetForm();
      this.showModal = true;
    },

    openEditModal(vehicle) {
      this.isEditMode = true;
      this.editingVehicleId = vehicle.id;
      this.formData = { ...vehicle };
      this.showModal = true;
      this.showViewModal = false;
    },

    openViewModal(vehicle) {
      this.selectedVehicle = vehicle;
      this.showViewModal = true;
    },

    closeViewModal() {
      this.showViewModal = false;
      this.selectedVehicle = null;
    },

    openRemindersModal() {
      this.showRemindersModal = true;
    },

    closeRemindersModal() {
      this.showRemindersModal = false;
    },

    selectVehicleAndCloseReminders(vehicle) {
      this.selectedVehicle = vehicle;
      this.showRemindersModal = false;
      this.showViewModal = true;
    },

    closeModal() {
      this.showModal = false;
      this.resetForm();
    },

    resetForm() {
      this.formData = {
        manufacturer: '',
        modelName: '',
        registerNumber: '',
        owner: '',
        fuelType: '',
        transmissionType: '',
        mileage: 0,
        basePrice: 0,
        unitPrice: 0,
        additionalHourPrice: 0,
        distanceRange: 0,
        revenueLicenseDate: '',
        insuranceDate: '',
      };
    },

    async saveVehicle() {
      // Validate required fields
      if (!this.formData.manufacturer ||
          !this.formData.modelName ||
          !this.formData.registerNumber ||
          !this.formData.owner ||
          !this.formData.fuelType ||
          !this.formData.transmissionType ) {
        showError('Please fill all required fields');
        return;
      }

      if (this.formData.basePrice < 0 || this.formData.unitPrice < 0 || this.formData.additionalHourPrice < 0 || this.formData.distanceRange < 0) {
        showError('Prices cannot be negative');
        return;
      }

      const vehicleData = {
        vehicle_id: this.formData.registerNumber.replaceAll("-", ""),
        manufacturer: this.formData.manufacturer,
        model_name: this.formData.modelName,
        register_number: this.formData.registerNumber,
        owner: this.formData.owner,
        fuel_type: this.formData.fuelType,
        transmission_type: this.formData.transmissionType,
        mileage: this.formData.mileage,
        base_price: this.formData.basePrice,
        unit_price: this.formData.unitPrice,
        addition_hour_price: this.formData.additionalHourPrice,
        distance_range: this.formData.distanceRange,
        revenue_licence_date: this.formData.revenueLicenseDate,
        insurance_date: this.formData.insuranceDate,
        created_by: this.loggedUser,
      };

      try {
        if (this.isEditMode) {

          const updateVehicleData = {
            vehicle_id: this.formData.registerNumber.replaceAll("-", ""),
            owner: this.formData.owner,
            base_price: this.formData.basePrice,
            unit_price: this.formData.unitPrice,
            addition_hour_price: this.formData.additionalHourPrice,
            distance_range: this.formData.distanceRange,
            revenue_licence_date: this.formData.revenueLicenseDate,
            insurance_date: this.formData.insuranceDate,
          }
          // Update vehicle
          vehicleData.id = this.editingVehicleId;
          await dbService.updateVehicle(updateVehicleData);

          // Update local array
          const index = this.vehicles.findIndex(v => v.id === this.editingVehicleId);
          if (index > -1) {
            this.vehicles[index] = { id: this.editingVehicleId, ...this.formData };
          }
        } else {
          // Create new vehicle
          console.log(vehicleData)
          const result = await dbService.createVehicle(vehicleData);
          this.vehicles.push({
            id: result.id,
            ...this.formData
          });
        }

        this.closeModal();
        showSuccess(this.isEditMode ? 'Vehicle updated successfully!' : 'Vehicle added successfully!');
      } catch (error) {
        console.log('Error saving vehicle:', error);
        showError('Error saving vehicle. Please try again.');
      }
    },
    async deleteVehicle(vehicleId) {
      this.$refs.confirmDialog.open({
        title: 'Removed Vehicle',
        subtitle: 'This cannot be modified or redo',
        message: 'Are you sure you want to delete this vehicle?',
        type: 'error',
        confirmText: 'Yes',
        onConfirm: async () => {
          await this.deleteVehicleConfirm(vehicleId);
        },
      })
    },
    async deleteVehicleConfirm(vehicleId) {
      this.confirmDialog.open({
        title: 'Delete Vehicle',
        subtitle: 'This will be effect to other',
        message: 'Are you sure you want to delete this vehicle?',
        type: 'error',
        confirmText: 'Delete',
        onConfirm: async () => {
          await dbService.deleteVehicle(vehicleId);

          const index = this.vehicles.findIndex(v => v.vehicleId === vehicleId);
          if (index > -1) {
            this.vehicles.splice(index, 1);
          }
        },
      })
    },

    handleLogout() {
      this.$emit('logout');
    },

    async getVehicles() {
      try {
        const result = await dbService.getVehicles();
        this.vehicles = result.map((item) => ({
          id: item.id,
          vehicleId: item.vehicle_id,
          manufacturer: item.manufacturer,
          modelName: item.model_name,
          registerNumber: item.register_number,
          owner: item.owner,
          fuelType: item.fuel_type,
          transmissionType: item.transmission_type,
          mileage: item.mileage,
          basePrice: item.base_price,
          unitPrice: item.unit_price,
          additionalHourPrice: item.addition_hour_price,
          distanceRange: item.distance_range,
          revenueLicenseDate: item.revenue_license_date,
          insuranceDate: item.insurance_date,
        }));
      } catch (error) {
        console.log('Error fetching vehicles:', error);
      }
    },
    registerNumberValidation(plateNumber){
      if (!plateNumber) return false;

      // Clean input: Convert to uppercase and trim spaces from ends
      const cleanedPlate = plateNumber.toUpperCase().trim();

      // Regex Explanation:
      // ^(?:[A-Z]{2}\s)?  --> Optional 2-letter province code + space (e.g., "WP ")
      // (
      //   [A-Z]{2,3}-\d{4} --> Format 1 & 2: 2 or 3 letters, a dash, and 4 digits (e.g., CAA-1234, GA-1234)
      //   |                --> OR
      //   \d{2,3}-\d{4}    --> Format 3: 2 or 3 digits, a dash, and 4 digits (e.g., 19-1234, 301-1234)
      // )$
      const sriLankaPlateRegex = /^(?:[A-Z]{2}\s)?([A-Z]{2,3}-\d{4}|\d{2,3}-\d{4})$/;

      return sriLankaPlateRegex.test(cleanedPlate) || 'Invalid register number';
    }
  },
  async mounted() {
    await this.getVehicles();
  }
};
</script>

<style scoped>
.vehicles-page {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
  background: var(--color-background-tertiary);
}

.vehicles-content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.vehicles-container {
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
  align-items: center;
}

.btn-reminders {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem 1.5rem;
  background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
  color: white;
  border: none;
  border-radius: var(--border-radius-md);
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all var(--transition-normal);
  box-shadow: 0 4px 12px rgba(245, 158, 11, 0.3);
  position: relative;
}

.btn-reminders:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(245, 158, 11, 0.4);
}

.btn-reminders i {
  font-size: 18px;
}

.reminder-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 24px;
  height: 24px;
  background: rgba(255, 255, 255, 0.3);
  border-radius: 50%;
  font-size: 12px;
  font-weight: 700;
  margin-left: 0.5rem;
}

.btn-add-vehicle {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem 1.5rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: var(--border-radius-md);
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all var(--transition-normal);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
}

.btn-add-vehicle:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(102, 126, 234, 0.4);
}

.btn-add-vehicle i {
  font-size: 18px;
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

.vehicles-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

.vehicles-table thead {
  position: sticky;
  top: 0;
  background: var(--color-background-secondary);
  border-bottom: 1px solid var(--color-border-tertiary);
  z-index: 10;
}

.vehicles-table th {
  padding: 0.75rem;
  text-align: left;
  font-weight: 500;
  color: var(--color-text-secondary);
  white-space: nowrap;
}

.vehicles-table td {
  padding: 0.75rem;
  border-bottom: 1px solid var(--color-border-tertiary);
  vertical-align: middle;
}

.vehicles-table tbody tr:hover {
  background: var(--color-background-secondary);
}

.vehicle-name {
  font-family: inherit;
  color: var(--color-text-primary);
  font-weight: 500;
}

.registration {
  color: var(--color-text-secondary);
  font-family: monospace;
}

.transmission {
  text-align: left;
  font-weight: 500;
}

.transmission-badge {
  display: inline-block;
  padding: 0.4rem 0.8rem;
  border-radius: 20px;
  font-size: 11px;
  font-weight: 500;
}

.transmission-badge.manual {
  background: #dbeafe;
  color: #1e40af;
  font-weight: 500;
}

.transmission-badge.automatic {
  background: #fce7f3;
  color: #831843;
  font-weight: 500;
}

.price {
  text-align: left;
  font-family: monospace;
  color: var(--color-success);
  font-weight: 500;
}

.actions {
  display: flex;
  gap: 0.5rem;
  justify-content: left;
  white-space: nowrap;
}

.btn-action {
  padding: 0.4rem 0.5rem;
  border: 1px solid var(--color-border-tertiary);
  background: transparent;
  color: var(--color-text-secondary);
  border-radius: 4px;
  cursor: pointer;
  transition: all var(--transition-fast);
  display: inline-flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
}

.btn-action:hover {
  border-color: var(--color-info);
  color: var(--color-info);
  background: rgba(59, 130, 246, 0.05);
}

.btn-action.view:hover {
  border-color: #8b5cf6;
  color: #8b5cf6;
  background: rgba(139, 92, 246, 0.05);
}

.btn-action.edit:hover {
  border-color: #f59e0b;
  color: #f59e0b;
  background: rgba(245, 158, 11, 0.05);
}

.btn-action.delete:hover {
  border-color: var(--color-danger);
  color: var(--color-danger);
  background: rgba(239, 68, 68, 0.05);
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

.empty-state p {
  margin: 0 0 1.5rem 0;
  font-size: 16px;
}

.btn-add-empty {
  padding: 0.75rem 1.5rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: var(--border-radius-md);
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all var(--transition-normal);
}

.btn-add-empty:hover {
  transform: translateY(-2px);
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
  font-weight: 500;
  color: var(--color-text-primary);
}

.stat-value.manual {
  color: #1e40af;
}

.stat-value.automatic {
  color: #831843;
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
  max-width: 600px;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
}

.modal-content.view-modal {
  max-width: 700px;
}

.modal-content.reminders-modal {
  max-width: 650px;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.5rem;
  border-bottom: 1px solid var(--color-border-tertiary);
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border-radius: var(--border-radius-lg) var(--border-radius-lg) 0 0;
}

.modal-header.reminders-header {
  background: linear-gradient(135deg, #f59e0b 0%, #d97706 100%);
}

.modal-header h2 {
  margin: 0;
  font-size: 20px;
  color: white;
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.modal-header h2 i {
  font-size: 22px;
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
  padding: 1.5rem;
}

.modal-body form {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1.5rem;
}

.form-group:nth-child(n+13) {
  grid-column: 1 / 2;
}

.modal-body form .form-actions {
  grid-column: 1 / -1;
}

.view-body {
  padding: 0;
}

.reminders-body {
  padding: 1.5rem;
}

.reminders-content {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.reminder-card {
  background: var(--color-background-secondary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-md);
  padding: 1.25rem;
  display: flex;
  flex-direction: column;
  gap: 1rem;
  transition: all var(--transition-normal);
}

.reminder-card:hover {
  border-color: #f59e0b;
  box-shadow: 0 4px 12px rgba(245, 158, 11, 0.1);
}

.reminder-vehicle-info {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
}

.vehicle-name-section {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.vehicle-name-section strong {
  font-size: 16px;
  color: var(--color-text-primary);
  font-weight: 600;
}

.vehicle-name-section .reg-number {
  font-size: 13px;
  color: var(--color-text-secondary);
  font-family: monospace;
  font-weight: 500;
}

.reminder-items {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.expiration-item {
  display: flex;
  gap: 1rem;
  padding: 0.75rem;
  border-radius: 6px;
  align-items: flex-start;
}

.expiration-item.revenue {
  background: rgba(59, 130, 246, 0.05);
  border: 1px solid rgba(59, 130, 246, 0.2);
}

.expiration-item.insurance {
  background: rgba(239, 68, 68, 0.05);
  border: 1px solid rgba(239, 68, 68, 0.2);
}

.expiration-item i {
  font-size: 18px;
  margin-top: 0.2rem;
  color: var(--color-text-secondary);
  min-width: 20px;
}

.expiration-item.revenue i {
  color: #3b82f6;
}

.expiration-item.insurance i {
  color: #ef4444;
}

.expiration-details {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
  flex: 1;
}

.expiration-details label {
  font-size: 12px;
  color: var(--color-text-secondary);
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.3px;
}

.expiration-details p {
  margin: 0;
  font-size: 14px;
  color: var(--color-text-primary);
  font-weight: 500;
  font-family: monospace;
}

.days-left {
  font-size: 12px;
  font-weight: 700;
  margin-top: 0.25rem;
}

.expiration-item.revenue .days-left {
  color: #3b82f6;
}

.expiration-item.insurance .days-left {
  color: #ef4444;
}

.btn-view-details {
  padding: 0.6rem 1rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 13px;
  font-weight: 500;
  transition: all var(--transition-fast);
  align-self: flex-start;
  margin-top: 0.5rem;
}

.btn-view-details:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
}

.reminders-footer {
  display: flex;
  justify-content: flex-end;
  padding: 1.5rem;
  background: var(--color-background-secondary);
  border-top: 1px solid var(--color-border-tertiary);
  border-radius: 0 0 var(--border-radius-lg) var(--border-radius-lg);
}

/* === VIEW MODAL STYLES === */
.details-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 2rem;
  padding: 2rem;
  border-bottom: 1px solid var(--color-border-tertiary);
}

.details-column {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.detail-item {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.detail-item label {
  font-size: 12px;
  color: var(--color-text-secondary);
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.detail-item p {
  margin: 0;
  font-size: 15px;
  color: var(--color-text-primary);
  font-weight: 500;
}

.detail-item .price-value {
  color: var(--color-success);
  font-family: monospace;
  font-weight: 600;
}

.view-actions {
  display: flex;
  gap: 1rem;
  justify-content: flex-end;
  padding: 1.5rem;
  background: var(--color-background-secondary);
  border-radius: 0 0 var(--border-radius-lg) var(--border-radius-lg);
}

/* === FORM STYLES === */
.form-group {
  margin-bottom: 1.5rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.6rem;
  font-size: 14px;
  font-weight: 600;
  color: var(--color-text-primary);
  letter-spacing: 0.3px;
  text-transform: capitalize;
}

.required {
  color: var(--color-danger);
  font-weight: 700;
  margin-left: 0.2rem;
}

.input-field {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid var(--color-border-tertiary);
  border-radius: 4px;
  font-size: 14px;
  color: var(--color-text-primary);
  background: var(--color-background-primary);
  transition: all var(--transition-fast);
  font-family: inherit;
}

.input-field:focus {
  outline: none;
  border-color: var(--color-info);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

/* === SELECT/DROPDOWN STYLING === */
select.input-field {
  cursor: pointer;
  padding-right: 2.5rem;
  appearance: none;
  background-image: url("data:image/svg+xml;charset=UTF-8,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='%23667eea' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3e%3cpolyline points='6 9 12 15 18 9'%3e%3c/polyline%3e%3c/svg%3e");
  background-repeat: no-repeat;
  background-position: right 0.75rem center;
  background-size: 1.25rem;
  padding-right: 2.5rem;
  color: var(--color-text-primary);
  transition: all var(--transition-fast);
}

select.input-field:hover:not(:disabled) {
  border-color: var(--color-info);
  background-color: rgba(102, 126, 234, 0.02);
  box-shadow: 0 2px 8px rgba(102, 126, 234, 0.1);
}

select.input-field:focus {
  outline: none;
  border-color: var(--color-info);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
  background-color: var(--color-background-primary);
}

select.input-field:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  background-color: var(--color-background-secondary);
}

select.input-field option {
  padding: 0.6rem 0.8rem;
  background: var(--color-background-primary);
  color: var(--color-text-primary);
  border: none;
  margin: 0.25rem 0;
}

select.input-field option:hover {
  background: #667eea;
  color: white;
}

select.input-field option:checked {
  background: linear-gradient(#667eea, #667eea);
  background-color: #667eea !important;
  color: white;
  font-weight: 500;
}

.form-actions {
  display: flex;
  gap: 1rem;
  justify-content: flex-end;
  padding-top: 1rem;
  border-top: 1px solid var(--color-border-tertiary);
  margin-top: 2rem;
}

.btn-secondary {
  padding: 0.75rem 1.5rem;
  border: 1px solid var(--color-border-tertiary);
  background: transparent;
  color: var(--color-text-primary);
  border-radius: var(--border-radius-md);
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all var(--transition-fast);
}

.btn-secondary:hover {
  background: var(--color-background-secondary);
}

.btn-primary {
  padding: 0.75rem 1.5rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: var(--border-radius-md);
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all var(--transition-fast);
}

.btn-primary:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
}

/* === RESPONSIVE === */
@media (max-width: 1024px) {
  .vehicles-container {
    padding: 1rem;
  }

  .vehicles-table {
    font-size: 12px;
  }

  .vehicles-table th,
  .vehicles-table td {
    padding: 0.6rem;
  }

  .btn-action {
    padding: 0.3rem 0.4rem;
    font-size: 12px;
  }

  .modal-body form {
    grid-template-columns: 1fr;
  }

  .page-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 0.75rem;
  }

  .header-actions {
    width: 100%;
    flex-direction: column;
  }

  .header-actions button {
    width: 100%;
  }
}

@media (max-width: 768px) {
  .page-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 1rem;
  }

  .header-actions {
    width: 100%;
    flex-direction: column;
    gap: 0.75rem;
  }

  .header-actions button {
    width: 100%;
  }

  .btn-add-vehicle {
    width: 100%;
  }

  .vehicles-table {
    font-size: 11px;
  }

  .vehicles-table th,
  .vehicles-table td {
    padding: 0.5rem;
  }

  .modal-content {
    width: 95%;
    max-width: 95%;
  }

  .summary-stats {
    grid-template-columns: repeat(2, 1fr);
  }

  .details-grid {
    grid-template-columns: 1fr;
    gap: 1rem;
    padding: 1rem;
  }

  .view-actions {
    flex-direction: column;
    gap: 0.75rem;
  }

  .view-actions button {
    width: 100%;
  }

  .modal-body form {
    grid-template-columns: 1fr;
  }

  select.input-field {
    padding-right: 2.5rem;
    background-position: right 0.75rem center;
  }

  .reminder-card {
    padding: 1rem;
  }

  .btn-view-details {
    width: 100%;
    text-align: center;
  }
}

/* Scrollbar styling */
.table-wrapper::-webkit-scrollbar,
.modal-content::-webkit-scrollbar {
  width: 6px;
}

.table-wrapper::-webkit-scrollbar-track,
.modal-content::-webkit-scrollbar-track {
  background: transparent;
}

.table-wrapper::-webkit-scrollbar-thumb,
.modal-content::-webkit-scrollbar-thumb {
  background: var(--color-border-secondary);
  border-radius: 3px;
}

.table-wrapper::-webkit-scrollbar-thumb:hover,
.modal-content::-webkit-scrollbar-thumb:hover {
  background: var(--color-border-primary);
}
</style>
