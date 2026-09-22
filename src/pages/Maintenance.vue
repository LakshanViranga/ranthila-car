<template>
  <div class="maintenance-page">
    <!-- Header Component -->
    <header-component
        :userName="`Logged in as ${loggedUser}`"
        :showBackButton="true"
        @logout="handleLogout"
    />

    <!-- Maintenance Content -->
    <div class="maintenance-content">
      <div class="maintenance-container">

        <!-- Page Header with Upcoming Badge Button -->
        <div class="page-header">
          <h1>Vehicle Maintenance</h1>
          <div class="header-actions">
            <button
                v-if="upcomingMaintenance.length > 0"
                class="btn-upcoming-badge"
                @click="showUpcomingPanel = !showUpcomingPanel"
                :class="{ active: showUpcomingPanel }"
            >
              <i class="fa fa-bell"></i>
              <span class="badge-text">Upcoming</span>
              <span class="badge-count">{{ upcomingMaintenance.length }}</span>
            </button>
            <button class="btn-add-record" @click="openAddModal">
              <i class="fa fa-plus"></i>
              <span>Add Maintenance Record</span>
            </button>
          </div>
        </div>

        <!-- Upcoming Events Sliding Panel -->
        <div v-if="showUpcomingPanel" class="upcoming-panel">
          <div class="panel-header">
            <h2>📌 Upcoming Maintenance (Next 7 Days)</h2>
            <button class="btn-close-panel" @click="showUpcomingPanel = false">
              <i class="fa fa-times"></i>
            </button>
          </div>
          <div class="panel-content">
            <div v-if="upcomingMaintenance.length > 0" class="reminders-grid">
              <div v-for="reminder in upcomingMaintenance" :key="reminder.id" class="reminder-card">
                <div class="reminder-header">
                  <div class="reminder-vehicle">
                    <h3>{{ reminder.vehicleName }}</h3>
                    <p class="register-number">{{ reminder.vehicleRegisterNumber }}</p>
                  </div>
                  <span :class="['urgency-badge', `urgency-${calculateUrgency(reminder.dueDate)}`]">
                    {{ getDaysUntil(reminder.dueDate) }}
                  </span>
                </div>
                <div class="reminder-content">
                  <p><strong>Service Type:</strong> {{ reminder.maintenanceRecord }}</p>
                  <p><strong>Description:</strong> {{ reminder.description }}</p>
                  <p><strong>Due Date:</strong> {{ formatDate(reminder.dueDate) }}</p>
                </div>
                <div class="reminder-actions">
                  <button class="btn-action view" @click="viewRecord(reminder)" title="View">
                    <i class="fa fa-eye"></i>
                  </button>
                  <button class="btn-action edit" @click="openNewRecordModal(reminder)" title="Complete">
                    <i class="fa fa-edit"></i>
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>

        <!-- Main Maintenance Table Section -->
        <div class="main-content">
          <div class="table-section">
            <div class="table-header">
              <h2 class="section-title">Maintenance History</h2>
              <div class="table-filters">
                <input
                    v-model="searchQuery"
                    type="text"
                    class="search-input"
                    placeholder="Search vehicle or service type..."
                />
              </div>
            </div>

            <div class="table-wrapper">
              <table class="maintenance-table">
                <thead>
                <tr>
                  <th>Register Number</th>
                  <th>Service Type</th>
                  <th>Description</th>
                  <th>Cost</th>
                  <th>Created Date</th>
                  <th>Payment Type</th>
                  <th>Payment Status</th>
                  <th>Actions</th>
                </tr>
                </thead>
                <tbody>
                <tr v-for="record in filteredRecords" :key="record.id" class="record-row">
                  <td class="register-number">
                    {{ record.vehicleRegisterNumber }}
                  </td>
                  <td class="service-type">
                    <span class="service-badge">{{ record.maintenanceRecord }}</span>
                  </td>
                  <td class="description">
                    {{ record.description }}
                  </td>
                  <td class="cost">
                    {{ formatPrice(record.cost) }}
                  </td>
                  <td class="created-date">
                    {{ formatDate(record.createdDate) }}
                  </td>
                  <td class="payment-type">
                    {{ record.paymentType}}
                  </td>
                  <td class="payment-status">
                      <span :class="['status-badge', `status-${record.paymentStatus.toLowerCase()}`]">
                        {{ record.paymentStatus }}
                      </span>
                  </td>
                  <td class="actions">
                    <button
                        class="btn-action view"
                        @click="viewRecord(record)"
                        title="View Details"
                    >
                      <i class="fa fa-eye"></i>
                    </button>
                    <button
                        class="btn-action edit"
                        @click="openEditModal(record)"
                        title="Edit"
                    >
                      <i class="fa fa-edit"></i>
                    </button>
                    <button
                        class="btn-action edit"
                        @click="completeMaintenanceRecord(record.id)"
                        title="Completed"
                    >
                      <i class="fa fa-check"></i>
                    </button>
                    <button
                        class="btn-action delete"
                        @click="deleteRecord(record.id)"
                        title="Delete"
                    >
                      <i class="fa fa-trash"></i>
                    </button>
                  </td>
                </tr>
                </tbody>
              </table>

              <!-- Empty State -->
              <div v-if="filteredRecords.length === 0" class="empty-state">
                <i class="fa fa-tools"></i>
                <p>No maintenance records found</p>
                <button class="btn-add-empty" @click="openAddModal">Add First Record</button>
              </div>
            </div>
          </div>

        </div>
      </div>
    </div>

    <!-- Add/Edit Maintenance Record Modal - IMPROVED VERSION -->
    <div v-if="showModal" class="modal-overlay" @click="closeModal">
      <div class="modal-content improved-modal" @click.stop>
        <div class="modal-header">
          <h2>{{ isEditMode ? 'Edit Maintenance Record' : 'Add Maintenance Record' }}</h2>
          <button class="btn-close" @click="closeModal">
            <i class="fa fa-times"></i>
          </button>
        </div>

        <div class="modal-body improved-modal-body">
          <form @submit.prevent="saveRecord">
            <!-- Section 1: Vehicle & Service Info -->
            <div class="form-section">
              <h3 class="section-label">Vehicle & Service</h3>

              <!-- Vehicle Selection -->
              <div class="form-group">
                <label for="vehicle">Vehicle <span v-if="!isEditMode" class="required">*</span></label>
                <select
                    id="vehicle"
                    v-model="formData.vehicleId"
                    class="input-field"
                    required
                    :disabled="isEditMode"
                    @change="onVehicleChange"
                >
                  <option value="">Select vehicle</option>
                  <option v-for="vehicle in vehicles" :key="vehicle.vehicle_id" :value="vehicle.vehicle_id">
                    {{ vehicle.registerNumber }} - {{ vehicle.manufacturer }} {{ vehicle.modelName }}
                  </option>
                </select>
              </div>

              <!-- Service Mileage (Current) -->
              <div class="form-group">
                <label for="serviceMileage">Current Mileage (km) <span class="required">*</span></label>
                <input
                    id="serviceMileage"
                    v-model.number="formData.serviceMileage"
                    type="number"
                    class="input-field"
                    placeholder="0"
                    min="0"
                    required
                />
              </div>

              <!-- Maintenance Record Type -->
              <div class="form-group">
                <label for="maintenanceType">Service Type <span class="required">*</span></label>
                <select
                    id="maintenanceType"
                    v-model="formData.maintenanceRecord"
                    class="input-field"
                    required
                >
                  <option value="">Select service type</option>
                  <option value="Oil Change">Oil Change</option>
                  <option value="Filter Replacement">Filter Replacement</option>
                  <option value="Tire Replacement">Tire Replacement</option>
                  <option value="Brake Service">Brake Service</option>
                  <option value="Engine Tune-up">Engine Tune-up</option>
                  <option value="Battery Replacement">Battery Replacement</option>
                  <option value="Transmission Service">Transmission Service</option>
                  <option value="AC Service">AC Service</option>
                  <option value="General Inspection">General Inspection</option>
                  <option value="Other">Other</option>
                </select>
              </div>
            </div>

            <!-- Section 2: Service Details -->
            <div class="form-section">
              <h3 class="section-label">Service Details</h3>

              <!-- Description -->
              <div class="form-group">
                <label for="description">Description <span v-if="!isEditMode" class="required">*</span></label>
                <textarea
                    id="description"
                    v-model="formData.description"
                    class="input-field textarea"
                    placeholder="Describe the maintenance work performed..."
                    rows="3"
                    required
                ></textarea>
              </div>

              <!-- Cost -->
              <div class="form-group">
                <label for="cost">Cost (LKR) <span v-if="!isEditMode" class="required">*</span></label>
                <input
                    id="cost"
                    v-model.number="formData.cost"
                    type="number"
                    class="input-field"
                    placeholder="0.00"
                    step="0.01"
                    min="0"
                    required
                    :readonly="isEditMode"
                />
              </div>

              <!-- Service Date -->
              <div class="form-group">
                <label for="serviceDate">Service Date <span v-if="!isEditMode" class="required">*</span></label>
                <input
                    id="serviceDate"
                    v-model="formData.serviceDate"
                    type="date"
                    class="input-field"
                    required
                    @change="validateServiceDate"
                />
              </div>

              <!-- Technician Name -->
              <div class="form-group">
                <label for="technician">Service Provider</label>
                <input
                    id="technician"
                    v-model="formData.technician"
                    type="text"
                    class="input-field"
                    placeholder="Enter technician name"
                />
              </div>
            </div>

            <!-- Section 3: Next Service Schedule (Conditional) -->
            <div class="form-section">
              <h3 class="section-label">Next Service Reminder</h3>

              <!-- Radio Buttons for Next Service Type -->
              <div class="form-group radio-group">
                <label class="radio-label">Track next service by:</label>
                <div class="radio-buttons-row">
                  <div class="radio-option" v-for="option in nextServiceOptions" :key="option.value">
                    <input
                        :id="`radio-${option.value}`"
                        v-model="formData.nextServiceType"
                        type="radio"
                        :value="option.value"
                        class="radio-input"
                    />
                    <label :for="`radio-${option.value}`" class="radio-text">
                      {{ option.label }}
                    </label>
                  </div>
                </div>
              </div>

              <!-- Due Date for Next Service (Show if: date or both) -->
              <div v-if="formData.nextServiceType === 'date' || formData.nextServiceType === 'both'" class="form-group">
                <label for="dueDate">Due Date for Next Service</label>
                <input
                    id="dueDate"
                    v-model="formData.dueDate"
                    type="date"
                    class="input-field"
                />
              </div>

              <!-- Due Mileage for Next Service (Show if: mileage or both) -->
              <div v-if="formData.nextServiceType === 'mileage' || formData.nextServiceType === 'both'" class="form-group">
                <label for="dueMileage">Due Mileage for Next Service (km)</label>
                <input
                    id="dueMileage"
                    v-model.number="formData.dueMileage"
                    type="number"
                    class="input-field"
                    placeholder="0"
                    min="0"
                />
              </div>
            </div>

            <!-- Section 4: Payment Info -->
            <div class="form-section">
              <h3 class="section-label">Payment Information</h3>

              <div class="form-group">
                <label for="paymentType">Payment Type <span v-if="!isEditMode" class="required">*</span></label>
                <v-select
                    v-model="formData.paymentType"
                    :items="paymentTypeArray"
                    item-title="title"
                    item-value="value"
                    class="input-field"
                    density="compact"
                    variant="outlined"
                    hide-details
                    placeholder="Select payment type"
                    :disabled="isEditMode"
                />
              </div>

              <div class="form-group">
                <label for="paymentStatus">Payment Status <span v-if="!isEditMode" class="required">*</span></label>
                <input
                    id="paymentStatus"
                    type="text"
                    :value="formData.paymentStatus"
                    class="input-field status-field"
                    readonly
                />
              </div>
            </div>

            <!-- Form Actions -->
            <div class="form-actions">
              <button type="button" class="btn-secondary" @click="closeModal">Cancel</button>
              <button type="submit" class="btn-primary">
                {{ isEditMode ? 'Update Record' : 'Add Record' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>

    <!-- View Record Modal -->
    <div v-if="showViewModal" class="modal-overlay" @click="closeViewModal">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h2>Maintenance Record Details</h2>
          <button class="btn-close" @click="closeViewModal">
            <i class="fa fa-times"></i>
          </button>
        </div>

        <div class="modal-body view-body" v-if="selectedRecord">
          <div class="details-section">
            <h3>Vehicle Information</h3>
            <div class="details-grid">
              <div class="detail-item">
                <label>Vehicle Name</label>
                <p>{{ getVehicleName(selectedRecord.vehicleId) }}</p>
              </div>
              <div class="detail-item">
                <label>Register Number</label>
                <p>{{ selectedRecord.vehicleRegisterNumber }}</p>
              </div>
              <div class="detail-item">
                <label>Service Type</label>
                <p>{{ selectedRecord.maintenanceRecord }}</p>
              </div>
              <div class="detail-item">
                <label>Cost</label>
                <p class="cost-value">{{ formatPrice(selectedRecord.cost) }}</p>
              </div>
            </div>
          </div>

          <div class="details-section">
            <h3>Service Details</h3>
            <div class="details-grid">
              <div class="detail-item">
                <label>Service Date</label>
                <p>{{ formatDate(selectedRecord.serviceDate) }}</p>
              </div>
              <div class="detail-item">
                <label>Due Date</label>
                <p>{{ selectedRecord.dueDate ? formatDate(selectedRecord.dueDate) : '-' }}</p>
              </div>
              <div class="detail-item">
                <label>Technician</label>
                <p>{{ selectedRecord.technician || '-' }}</p>
              </div>
              <div class="detail-item">
                <label>Created Date</label>
                <p>{{ formatDate(selectedRecord.createdDate) }}</p>
              </div>
            </div>
          </div>

          <div class="details-section full-width">
            <h3>Description</h3>
            <p class="description-text">{{ selectedRecord.description }}</p>
          </div>

          <div v-if="selectedRecord.parts" class="details-section full-width">
            <h3>Parts Used</h3>
            <p class="description-text">{{ selectedRecord.parts }}</p>
          </div>
        </div>

        <div class="modal-footer">
          <button class="btn-secondary" @click="closeViewModal">Close</button>
          <button class="btn-primary" @click="openEditModal(selectedRecord)">
            <i class="fa fa-edit"></i>
            Edit Record
          </button>
        </div>
      </div>
    </div>
    <ConfirmationModal ref="confirmDialog"/>
  </div>
</template>

<script setup>
import { ref, computed, watch, onMounted } from 'vue'
import HeaderComponent from '../component/Header.vue';
import { dbService } from '../services/db.ts';
import { useSnackbar } from '../composables/useSnackbar'
import { useAuthStore } from '../stores/auth.ts'
import {
  paymentStatus,
  roleTypes,
  convertSnakeCase,
  paymentTypes, paymentTypeArray
} from '../utils/constants.ts'
import {useRouter} from "vue-router";
import ConfirmationModal from "../component/ConfirmationModal.vue";

const emit = defineEmits(['logout'])

const router = useRouter()
const { showSuccess, showError } = useSnackbar()
const authStore = useAuthStore()

// Reactive State
const loggedUser = ref(authStore.username)
const roleName = ref(authStore.role)
const showModal = ref(false)
const showViewModal = ref(false)
const showUpcomingPanel = ref(false)
const isEditMode = ref(false)
const editingRecordId = ref(null)
const selectedRecord = ref(null)
const searchQuery = ref('')
const vehicles = ref( [])
const maintenanceRecords = ref([])

// Next Service Options - from constants (add this to your constants.ts)
const nextServiceOptions = ref([
  { value: 'none', label: 'None' },
  { value: 'mileage', label: 'Mileage' },
  { value: 'date', label: 'Date' },
  { value: 'both', label: 'Both' }
])

let formData = ref({
  vehicleId: '',
  maintenanceRecord: '',
  description: '',
  cost: 0,
  serviceDate: '',
  serviceMileage: 0,
  dueDate: '',
  dueMileage: '',
  technician: '',
  paymentType: '',
  paymentStatus: '',
  nextServiceType: 'none'
})
const confirmDialog = ref(null)

watch(
    () => formData.value.paymentType,
    (newValue) => {
      console.log(newValue)
      if (newValue === paymentTypes.cashPayment|| newValue === paymentTypes.cardPayment || newValue === paymentTypes.bankTransfer ) {
        formData.value.paymentStatus = paymentStatus.completed
      } else if (newValue === paymentTypes.creditPayment) {
        formData.value.paymentStatus = paymentStatus.pending
      } else {
        formData.value.paymentStatus = ''
      }
    }
)

const filteredRecords = computed(() => {
  if (!searchQuery.value) return maintenanceRecords.value;

  const query = searchQuery.value.toLowerCase();
  return maintenanceRecords.value.filter(record =>
      record.vehicleName.toLowerCase().includes(query) ||
      record.vehicleRegisterNumber.toLowerCase().includes(query) ||
      record.maintenanceRecord.toLowerCase().includes(query) ||
      record.description.toLowerCase().includes(query)
  );
})

const upcomingMaintenance = computed(() => {
  const today = new Date();
  return maintenanceRecords.value.filter(record => {
    if (!record.dueDate) return false;
    const dueDate = new Date(record.dueDate);
    return dueDate >= today && dueDate <= new Date(today.getTime() + 7 * 24 * 60 * 60 * 1000);
  })
      .sort((a, b) => new Date(a.dueDate) - new Date(b.dueDate));
})

const formatPrice = (price) => {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'LKR',
    minimumFractionDigits: 2,
    maximumFractionDigits: 2
  }).format(price || 0);
}

const formatDate = (dateString) => {
  const date = new Date(dateString);
  return date.toLocaleString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  });
}

const calculateUrgency = (dueDate) => {
  const today = new Date();
  const due = new Date(dueDate);
  const diffTime = due - today;
  const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));
  return diffDays <= 0 ? 'critical' : (diffDays <= 7 ? 'warning' : 'normal');
}

const getDaysUntil = (dueDate) => {
  const today = new Date();
  const due = new Date(dueDate);
  const diffTime = due - today;
  const diffDays = Math.ceil(diffTime / (1000 * 60 * 60 * 24));
  return diffDays <= 0 ? '⚠️ Overdue' : `📅 ${diffDays}d`;
}

const onVehicleChange = () => {
  // Fetch current mileage of selected vehicle
  const selectedVehicle = vehicles.value.find(v => v.vehicle_id === formData.value.vehicleId);
  if (selectedVehicle && selectedVehicle.mileage) {
    formData.value.serviceMileage = selectedVehicle.mileage;
  }
}

const validateServiceDate = () => {
  const today = new Date();
  today.setHours(0, 0, 0, 0);
  const dateToCheck = new Date(formData.value.serviceDate);
  dateToCheck.setHours(0, 0, 0, 0);

  if (dateToCheck > today) {
    showError('Service date cannot be in the future');
    formData.value.serviceDate = '';
  }
}

const openAddModal = () => {
  isEditMode.value = false;
  editingRecordId.value = null;
  resetForm();
  showModal.value = true;
}

const openNewRecordModal = (record) => {
  isEditMode.value = false;
  editingRecordId.value = null;
  formData.value = {
    vehicleId: record.vehicleId,
    maintenanceRecord: record.maintenanceRecord,
    description: '',
    cost: 0,
    serviceDate: '',
    serviceMileage: 0,
    dueDate: '',
    dueMileage: '',
    technician: '',
    paymentType: '',
    paymentStatus: '',
    nextServiceType: 'none'
  };
  showModal.value = true;
}

const openEditModal = (record) =>{
  isEditMode.value = true;
  editingRecordId.value = record.id;
  formData.value = {
    vehicleId: record.vehicleId,
    maintenanceRecord: record.maintenanceRecord,
    description: record.description,
    cost: record.cost,
    serviceDate: record.serviceDate,
    serviceMileage: record.serviceMileage,
    dueDate: record.dueDate || '',
    dueMileage: record.dueMileage || '',
    technician: record.technician || '',
    paymentType: record.paymentType || '',
    paymentStatus: record.paymentStatus || '',
    nextServiceType: determineNextServiceType(record)
  };
  showModal.value = true;
  showViewModal.value = false;
}

const determineNextServiceType = (record) => {
  if (record.dueDate && record.dueMileage) return 'both';
  if (record.dueDate) return 'date';
  if (record.dueMileage) return 'mileage';
  return 'none';
}

const closeModal = () =>  {
  showModal.value = false;
  resetForm();
}

const resetForm = () => {
  formData.value = {
    vehicleId: '',
    maintenanceRecord: '',
    description: '',
    cost: 0,
    serviceDate: '',
    serviceMileage: 0,
    dueDate: '',
    dueMileage: '',
    technician: '',
    paymentStatus: '',
    paymentType: '',
    nextServiceType: 'none'
  };
}

const saveRecord = async () => {
  // Validate required fields
  if (!formData.value.vehicleId || !formData.value.maintenanceRecord ||
      !formData.value.description || !formData.value.cost || !formData.value.serviceDate || !formData.value.serviceMileage) {
    alert('Please fill all required fields');
    return;
  }

  const recordData = {
    vehicle_id: formData.value.vehicleId,
    service_type: formData.value.maintenanceRecord,
    description: formData.value.description,
    cost: formData.value.cost,
    service_date: formData.value.serviceDate,
    service_mileage: formData.value.serviceMileage,
    next_service_date: (formData.value.nextServiceType === 'date' || formData.value.nextServiceType === 'both') ? formData.value.dueDate || null : null,
    next_service_mileage: (formData.value.nextServiceType === 'mileage' || formData.value.nextServiceType === 'both') ? formData.value.dueMileage || null : null,
    payment_type: formData.value.paymentType,
    payment_status: formData.value.paymentStatus,
    service_provider: formData.value.technician || '',
    created_by: loggedUser.value
  };

  try {
    if (isEditMode.value) {
      recordData.id = editingRecordId.value;
      confirmDialog.value.open({
        title: 'Update Maintenance Record',
        subtitle: 'This will be effect to other',
        message: 'Are you sure you want to update this record?',
        type: 'warning',
        confirmText: 'Yes',
        onConfirm: async () => {
          await dbService.updateMaintenanceRecords(recordData);
          await getMaintenanceRecords();
        },
      })
    } else {
      await dbService.addMaintenance(recordData);
      await getMaintenanceRecords();
    }

    alert(isEditMode.value ? 'Record updated successfully!' : 'Record added successfully!');
    closeModal();
  } catch (error) {
    console.log('Error saving record:', error);
    alert('Error saving record. Please try again.');
  }
}

const viewRecord = (record) => {
  selectedRecord.value = record;
  showViewModal.value = true;
}

const closeViewModal = () => {
  showViewModal.value = false;
  selectedRecord.value = null;
}

const deleteRecord = async (recordId) => {
  confirmDialog.value.open({
    title: 'Delete Maintenance Record',
    subtitle: 'This will be effect to other',
    message: 'Are you sure you want to delete this record?',
    type: 'error',
    confirmText: 'Delete',
    onConfirm: async () => {
      await dbService.deleteMaintenanceRecords(recordId);
    },
  })
}

const completeMaintenanceRecord = async (recordId) => {
  try {
    await dbService.completeMaintenance({
      id: recordId,
      payment_status: paymentStatus.completed
    })

    await getMaintenanceRecords();
    showSuccess('Maintenance completed successfully!');
  }catch(error) {
    showError('Error maintenance record. Please try again.');
  }
}

const handleLogout = () => {
  emit('logout');
}

const getVehicles = async () => {
  try {
    const result = await dbService.getVehicles();
    vehicles.value = result.map((item) => ({
      id: item.id,
      vehicle_id: item.vehicle_id,
      registerNumber: item.register_number,
      manufacturer: item.manufacturer,
      modelName: item.model_name,
      mileage: item.mileage || 0 // Add current mileage from DB
    }));
    console.log(vehicles.value);
  } catch (error) {
    console.log('Error fetching vehicles:', error);
  }
}

const getMaintenanceRecords = async () => {
  try {
    const result = await dbService.getMaintenanceRecords();
    maintenanceRecords.value = result.map((item) => ({
      id: item.id,
      vehicleId: item.vehicle_id,
      vehicleRegisterNumber: item.vehicle_register_number,
      maintenanceRecord: item.service_type,
      description: item.description,
      cost: item.cost,
      serviceMileage: item.service_mileage,
      dueDate: item.next_service_date,
      dueMileage: item.next_service_mileage,
      technician: item.service_provider,
      paymentType: item.payment_type,
      paymentStatus: item.payment_status,
      createdBy: item.created_by,
      createdDate: item.created_at
    }));
  } catch (error) {
    console.log('Error fetching maintenance records:', error);
  }
}

const getVehicleName = async (vehicleId) => {
  const vehicle = vehicles.value.find(v => v.vehicle_id === vehicleId);
  return vehicle ? vehicle.manufacturer + vehicle.modelName : "Unknown";
}

// Lifecycle
onMounted(async () => {
  await Promise.all([getVehicles(), getMaintenanceRecords()])
})

</script>

<style scoped>
.maintenance-page {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
  background: var(--color-background-tertiary);
}

.maintenance-content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.maintenance-container {
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
  align-items: center;
  margin-bottom: 1.5rem;
  flex-wrap: wrap;
  gap: 1rem;
}

.page-header h1 {
  margin: 0;
  font-size: 28px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.header-actions {
  display: flex;
  align-items: center;
  gap: 1rem;
  flex-wrap: wrap;
}

.btn-add-record {
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
  font-weight: 600;
  transition: all var(--transition-normal);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
}

.btn-add-record:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(102, 126, 234, 0.4);
}

/* === UPCOMING BADGE BUTTON === */
.btn-upcoming-badge {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.6rem 1rem;
  background: linear-gradient(135deg, #f97316 0%, #ea580c 100%);
  color: white;
  border: none;
  border-radius: var(--border-radius-md);
  cursor: pointer;
  font-size: 13px;
  font-weight: 600;
  transition: all var(--transition-normal);
  box-shadow: 0 4px 12px rgba(249, 115, 22, 0.3);
  position: relative;
}

.btn-upcoming-badge:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(249, 115, 22, 0.4);
}

.btn-upcoming-badge.active {
  background: linear-gradient(135deg, #dc2626 0%, #991b1b 100%);
  box-shadow: 0 6px 20px rgba(220, 38, 38, 0.4);
}

.btn-upcoming-badge i {
  font-size: 16px;
}

.badge-text {
  display: none;
}

.badge-count {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 24px;
  height: 24px;
  padding: 0 6px;
  background: rgba(255, 255, 255, 0.25);
  border-radius: 12px;
  font-size: 12px;
  font-weight: 700;
}

/* === UPCOMING PANEL === */
.upcoming-panel {
  background: var(--color-background-primary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-lg);
  margin-bottom: 1.5rem;
  overflow: hidden;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
  animation: slideDown 0.3s ease-out;
}

@keyframes slideDown {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.25rem 1.5rem;
  background: linear-gradient(135deg, #f97316 0%, #ea580c 100%);
  color: white;
  border-bottom: 1px solid var(--color-border-tertiary);
}

.panel-header h2 {
  margin: 0;
  font-size: 15px;
  font-weight: 700;
  color: white;
}

.btn-close-panel {
  padding: 0.4rem;
  border: none;
  background: rgba(255, 255, 255, 0.2);
  color: white;
  cursor: pointer;
  font-size: 18px;
  border-radius: 4px;
  transition: all var(--transition-fast);
}

.btn-close-panel:hover {
  background: rgba(255, 255, 255, 0.3);
}

.panel-content {
  padding: 1.5rem;
  max-height: 400px;
  overflow-y: auto;
}

.panel-content::-webkit-scrollbar {
  width: 6px;
}

.panel-content::-webkit-scrollbar-thumb {
  background: var(--color-border-secondary);
  border-radius: 3px;
}

/* === MAIN CONTENT === */
.main-content {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  flex: 1;
  min-height: 0;
}

/* === REMINDERS SECTION === */
.reminders-section {
  margin-bottom: 0;
}

.section-title {
  margin: 0 0 1.25rem 0;
  font-size: 16px;
  font-weight: 700;
  color: var(--color-text-primary);
  letter-spacing: 0.5px;
}

.reminders-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 1rem;
}

.reminder-card {
  background: var(--color-background-secondary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-md);
  padding: 1rem;
  transition: all var(--transition-normal);
}

.reminder-card:hover {
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
  transform: translateY(-1px);
  border-color: #f97316;
}

.reminder-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 0.75rem;
  padding-bottom: 0.75rem;
  border-bottom: 1px solid var(--color-border-tertiary);
  gap: 0.75rem;
}

.reminder-vehicle h3 {
  margin: 0;
  font-size: 14px;
  font-weight: 700;
  color: var(--color-text-primary);
}

.reminder-vehicle .register-number {
  margin: 0.25rem 0 0 0;
  font-size: 12px;
  color: var(--color-text-secondary);
  font-family: monospace;
}

.urgency-badge {
  display: inline-block;
  padding: 0.4rem 0.8rem;
  border-radius: 20px;
  font-size: 12px;
  font-weight: 700;
  white-space: nowrap;
  flex-shrink: 0;
}

.urgency-badge.urgency-critical {
  background: #fee2e2;
  color: #dc2626;
}

.urgency-badge.urgency-warning {
  background: #fef3c7;
  color: #d97706;
}

.urgency-badge.urgency-normal {
  background: #dbeafe;
  color: #1e40af;
}

.reminder-content {
  display: flex;
  flex-direction: column;
  gap: 0.3rem;
  margin-bottom: 0.75rem;
}

.reminder-content p {
  margin: 0;
  font-size: 11px;
  color: var(--color-text-primary);
  line-height: 1.3;
}

.reminder-content strong {
  color: var(--color-text-secondary);
  font-weight: 600;
  font-size: 10px;
}

.reminder-actions {
  display: flex;
  gap: 0.5rem;
  justify-content: flex-end;
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

/* === TABLE SECTION === */
.table-section {
  background: var(--color-background-primary);
  border-radius: var(--border-radius-lg);
  border: 1px solid var(--color-border-tertiary);
  overflow: hidden;
  display: flex;
  flex-direction: column;
  flex: 1;
  min-height: 0;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
}

.table-header {
  padding: 1.5rem;
  border-bottom: 1px solid var(--color-border-tertiary);
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 1rem;
  flex-shrink: 0;
}

.table-filters {
  flex: 1;
  min-width: 200px;
}

.search-input {
  width: 100%;
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

.table-wrapper {
  flex: 1;
  overflow-y: auto;
  overflow-x: auto;
  min-height: 0;
}

.maintenance-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

.maintenance-table thead {
  position: sticky;
  top: 0;
  background: var(--color-background-secondary);
  border-bottom: 1px solid var(--color-border-tertiary);
  z-index: 10;
}

.maintenance-table th {
  padding: 0.75rem;
  text-align: left;
  font-weight: 600;
  color: var(--color-text-secondary);
  text-transform: capitalize;
  white-space: nowrap;
}

.maintenance-table td {
  padding: 0.75rem;
  border-bottom: 1px solid var(--color-border-tertiary);
  vertical-align: middle;
}

.maintenance-table tbody tr:hover {
  background: var(--color-background-secondary);
}

.vehicle-name {
  font-weight: 600;
  color: var(--color-text-primary);
}

.maintenance-table .register-number {
  font-family: monospace;
  color: var(--color-text-secondary);
}

.service-type {
  text-align: left;
}

.service-badge {
  display: inline-block;
  padding: 0.3rem 0.7rem;
  background: #dbeafe;
  color: #1e40af;
  border-radius: 12px;
  font-size: 11px;
  font-weight: 600;
  white-space: nowrap;
}

.description {
  max-width: 150px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: var(--color-text-secondary);
}

.cost {
  text-align: left;
  font-family: monospace;
  color: var(--color-success);
  font-weight: 600;
  white-space: nowrap;
}

.created-date {
  color: var(--color-text-secondary);
  font-size: 12px;
  white-space: nowrap;
}

.payment-type {
  color: var(--color-text-secondary);
  white-space: nowrap;
}

.payment-status {
  text-align: center;
}

.status-badge {
  display: inline-block;
  padding: 0.4rem 0.8rem;
  border-radius: 12px;
  font-size: 11px;
  font-weight: 700;
  text-transform: capitalize;
  white-space: nowrap;
}

.status-badge.status-completed {
  background: #d1fae5;
  color: #047857;
}

.status-badge.status-pending {
  background: #fed7aa;
  color: #b45309;
}

.status-badge.status-failed {
  background: #fee2e2;
  color: #dc2626;
}

.status-badge.status-partial {
  background: #dbeafe;
  color: #1e40af;
}

.actions {
  display: flex;
  gap: 0.5rem;
  justify-content: left;
  flex-wrap: wrap;
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
  font-weight: 600;
  transition: all var(--transition-normal);
}

.btn-add-empty:hover {
  transform: translateY(-2px);
}

/* === STATS SECTION === */
.stats-section {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 1rem;
  flex-shrink: 0;
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
  font-weight: 600;
}

.stat-value {
  font-size: 24px;
  font-weight: 700;
  color: var(--color-text-primary);
}

.stat-value.upcoming {
  color: #d97706;
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
  padding: 1rem;
}

.modal-content {
  background: var(--color-background-primary);
  border-radius: var(--border-radius-lg);
  width: 90%;
  max-width: 650px;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
}

/* === IMPROVED MODAL SPECIFIC STYLES === */
.improved-modal {
  max-width: 600px;
  max-height: 85vh;
}

.improved-modal-body {
  padding: 1.5rem;
  overflow-y: auto;
  max-height: calc(85vh - 130px);
}

.improved-modal-body::-webkit-scrollbar {
  width: 6px;
}

.improved-modal-body::-webkit-scrollbar-thumb {
  background: var(--color-border-secondary);
  border-radius: 3px;
}

.form-section {
  margin-bottom: 1.75rem;
  padding-bottom: 1.75rem;
  border-bottom: 1px solid var(--color-border-tertiary);
}

.form-section:last-of-type {
  border-bottom: none;
  margin-bottom: 0;
  padding-bottom: 0;
}

.section-label {
  margin: 0 0 1rem 0;
  font-size: 13px;
  font-weight: 700;
  text-transform: uppercase;
  color: var(--color-text-secondary);
  letter-spacing: 0.5px;
}

/* Radio Button Group Styles */
.radio-group {
  margin-bottom: 1.25rem;
}

.radio-label {
  display: block;
  margin-bottom: 0.75rem;
  font-size: 14px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.radio-buttons-row {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(100px, 1fr));
  gap: 0.75rem;
}

.radio-option {
  display: flex;
  align-items: center;
}

.radio-input {
  cursor: pointer;
  margin-right: 0.5rem;
  width: 18px;
  height: 18px;
  accent-color: #667eea;
}

.radio-text {
  cursor: pointer;
  font-size: 13px;
  color: var(--color-text-primary);
  font-weight: 500;
  user-select: none;
}

.radio-option:hover .radio-text {
  color: #667eea;
}

/* === FORM STYLES === */
.form-group {
  margin-bottom: 1rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.required {
  color: var(--color-danger);
  font-weight: 700;
}

.input-field {
  width: 100%;
  padding: 0.65rem 0.75rem;
  border: 1px solid var(--color-border-tertiary);
  border-radius: 4px;
  font-size: 13px;
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

.input-field.textarea {
  resize: vertical;
  min-height: 70px;
  font-size: 13px;
}

select.input-field {
  cursor: pointer;
  appearance: none;
  background-image: url("data:image/svg+xml;charset=UTF-8,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='%23667eea' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3e%3cpolyline points='6 9 12 15 18 9'%3e%3c/polyline%3e%3c/svg%3e");
  background-repeat: no-repeat;
  background-position: right 0.75rem center;
  background-size: 1.25rem;
  padding-right: 2.5rem;
}

.form-actions {
  display: flex;
  gap: 0.75rem;
  justify-content: flex-end;
  padding-top: 1.5rem;
  margin-top: 1.5rem;
  border-top: 1px solid var(--color-border-tertiary);
}

.btn-secondary {
  padding: 0.65rem 1.25rem;
  border: 1px solid var(--color-border-tertiary);
  background: transparent;
  color: var(--color-text-primary);
  border-radius: var(--border-radius-md);
  cursor: pointer;
  font-size: 13px;
  font-weight: 600;
  transition: all var(--transition-fast);
}

.btn-secondary:hover {
  background: var(--color-background-secondary);
}

.btn-primary {
  padding: 0.65rem 1.25rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: var(--border-radius-md);
  cursor: pointer;
  font-size: 13px;
  font-weight: 600;
  transition: all var(--transition-fast);
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.btn-primary:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.25rem 1.5rem;
  border-bottom: 1px solid var(--color-border-tertiary);
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border-radius: var(--border-radius-lg) var(--border-radius-lg) 0 0;
}

.modal-header h2 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
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

.view-body {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}

.details-section {
  padding: 1.5rem;
  background: var(--color-background-secondary);
  border-radius: 8px;
}

.details-section h3 {
  margin: 0 0 1rem 0;
  font-size: 14px;
  font-weight: 700;
  color: var(--color-text-primary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.details-section.full-width {
  grid-column: 1 / -1;
}

.details-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
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
  letter-spacing: 0.3px;
}

.detail-item p {
  margin: 0;
  font-size: 14px;
  color: var(--color-text-primary);
  font-weight: 500;
}

.detail-item .cost-value {
  color: var(--color-success);
  font-family: monospace;
  font-weight: 700;
}

.description-text {
  margin: 0;
  font-size: 13px;
  color: var(--color-text-primary);
  line-height: 1.6;
}

.modal-footer {
  display: flex;
  gap: 1rem;
  padding: 1.5rem;
  border-top: 1px solid var(--color-border-tertiary);
  background: var(--color-background-secondary);
  border-radius: 0 0 var(--border-radius-lg) var(--border-radius-lg);
  justify-content: flex-end;
}

.status-field {
  background: var(--color-background-secondary);
  cursor: not-allowed;
}

/* === RESPONSIVE === */
@media (max-width: 1024px) {
  .badge-text {
    display: inline;
  }

  .maintenance-table {
    font-size: 12px;
  }

  .maintenance-table th,
  .maintenance-table td {
    padding: 0.6rem;
  }

  .stats-section {
    grid-template-columns: repeat(4, 1fr);
  }

  .reminders-grid {
    grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  }
}

@media (max-width: 768px) {
  .maintenance-container {
    padding: 1rem;
  }

  .page-header {
    flex-direction: column;
    align-items: stretch;
    margin-bottom: 1rem;
  }

  .page-header h1 {
    font-size: 22px;
    margin-bottom: 0.5rem;
  }

  .header-actions {
    width: 100%;
  }

  .btn-add-record {
    flex: 1;
  }

  .btn-upcoming-badge {
    flex: 0.5;
  }

  .badge-text {
    display: none;
  }

  .table-header {
    flex-direction: column;
    align-items: flex-start;
  }

  .table-filters {
    width: 100%;
  }

  .search-input {
    width: 100%;
  }

  .maintenance-table {
    font-size: 11px;
  }

  .maintenance-table th,
  .maintenance-table td {
    padding: 0.5rem;
  }

  .modal-content {
    width: 95%;
  }

  .improved-modal {
    max-width: 95%;
  }

  .stats-section {
    grid-template-columns: repeat(2, 1fr);
  }

  .modal-footer {
    flex-direction: column;
  }

  .modal-footer button {
    width: 100%;
  }

  .form-actions {
    flex-direction: column;
    gap: 0.5rem;
  }

  .form-actions button {
    width: 100%;
  }

  .panel-content {
    max-height: 500px;
  }

  .reminders-grid {
    grid-template-columns: 1fr;
  }

  .reminder-header {
    flex-direction: row;
  }

  .reminder-content p {
    font-size: 11px;
  }

  .panel-header h2 {
    font-size: 14px;
  }

  .radio-buttons-row {
    grid-template-columns: repeat(2, 1fr);
  }

  .improved-modal-body {
    padding: 1rem;
  }
}

@media (max-width: 480px) {
  .page-header h1 {
    font-size: 18px;
  }

  .header-actions {
    flex-direction: column;
  }

  .btn-add-record,
  .btn-upcoming-badge {
    width: 100%;
  }

  .stats-section {
    grid-template-columns: 1fr;
  }

  .reminder-card {
    padding: 0.75rem;
  }

  .reminder-header {
    margin-bottom: 0.5rem;
    padding-bottom: 0.5rem;
  }

  .reminder-content {
    gap: 0.2rem;
    margin-bottom: 0.5rem;
  }

  .reminder-content p {
    font-size: 10px;
  }

  .reminder-actions {
    gap: 0.3rem;
  }

  .btn-action {
    padding: 0.3rem 0.4rem;
    font-size: 12px;
  }

  .radio-buttons-row {
    grid-template-columns: 1fr;
  }

  .form-group label {
    font-size: 12px;
  }

  .input-field {
    font-size: 12px;
    padding: 0.6rem;
  }
}

/* Scrollbar styling */
.maintenance-container::-webkit-scrollbar,
.table-wrapper::-webkit-scrollbar,
.modal-content::-webkit-scrollbar,
.panel-content::-webkit-scrollbar {
  width: 6px;
}

.maintenance-container::-webkit-scrollbar-track,
.table-wrapper::-webkit-scrollbar-track,
.modal-content::-webkit-scrollbar-track,
.panel-content::-webkit-scrollbar-track {
  background: transparent;
}

.maintenance-container::-webkit-scrollbar-thumb,
.table-wrapper::-webkit-scrollbar-thumb,
.modal-content::-webkit-scrollbar-thumb,
.panel-content::-webkit-scrollbar-thumb {
  background: var(--color-border-secondary);
  border-radius: 3px;
}

.maintenance-container::-webkit-scrollbar-thumb:hover,
.table-wrapper::-webkit-scrollbar-thumb:hover,
.modal-content::-webkit-scrollbar-thumb:hover,
.panel-content::-webkit-scrollbar-thumb:hover {
  background: var(--color-border-primary);
}
</style>
