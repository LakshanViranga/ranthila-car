<template>
  <div class="incident-page">
    <!-- Header -->
    <header-component
        :userName="`Logged in as ${loggedUser}`"
        :showBackButton="true"
        @logout="handleLogout"
    />

    <div class="incident-content">
      <div class="incident-container">

        <!-- Page Header -->
        <div class="page-header">
          <div>
            <h1 v-text="'Incident Management'"></h1>
            <p class="page-subtitle" v-text="'Track and manage vehicle rental incidents'"></p>
          </div>
        </div>

        <!-- Customer Search Section -->
        <div class="search-section">
          <div class="search-card">
            <h2 class="section-title" v-text="'Search Customer'"></h2>
            <p class="section-subtitle" v-text="'Enter customer ID to find their orders and incidents'"></p>

            <div class="search-container">
              <input
                  v-model="searchCustomerId"
                  type="text"
                  class="search-input-large"
                  placeholder="Enter Customer ID..."
                  @keyup.enter="searchCustomer"
              />
              <button class="btn-search" @click="searchCustomer">
                <i class="fa fa-search"></i>
                <span v-text="'Search'"></span>
              </button>
            </div>

            <div v-if="searchError" class="error-message">
              <i class="fa fa-exclamation-circle"></i>
              <span v-text="searchError"></span>
            </div>
          </div>
        </div>

        <!-- Customer Info & Order Selection -->
        <div v-if="selectedCustomer" class="customer-section">
          <div class="customer-info-card">
            <div class="customer-header">
              <div>
                <h3 class="customer-name" v-text="selectedCustomer.customerName"></h3>
                <p class="customer-id" v-text="`ID: ${selectedCustomer.customerId}`"></p>
              </div>
              <div class="customer-status">
                <span v-if="selectedCustomer.isBlacklisted" class="blacklist-badge">
                  <i class="fa fa-ban"></i>
                  <span v-text="'Blacklisted'"></span>
                </span>
                <span v-else class="active-badge">
                  <i class="fa fa-check-circle"></i>
                  <span v-text="'Active'"></span>
                </span>
              </div>
            </div>

            <div class="customer-details">
              <div class="detail-item">
                <span class="detail-label" v-text="'Contact:'"></span>
                <span class="detail-value" v-text="selectedCustomer.contactNumber || '-'"></span>
              </div>
              <div class="detail-item">
                <span class="detail-label" v-text="'License:'"></span>
                <span class="detail-value" v-text="selectedCustomer.licenseNumber || '-'"></span>
              </div>
            </div>

            <div class="customer-actions">
              <button
                  v-if="!selectedCustomer.isBlacklisted"
                  class="btn-blacklist"
                  @click="openBlacklistModal"
              >
                <i class="fa fa-ban"></i>
                <span v-text="'Add to Blacklist'"></span>
              </button>
              <button
                  v-else
                  class="btn-remove-blacklist"
                  @click="removeFromBlacklist"
              >
                <i class="fa fa-check"></i>
                <span v-text="'Remove from Blacklist'"></span>
              </button>
              <button class="btn-new-search" @click="newSearch">
                <i class="fa fa-search"></i>
                <span v-text="'New Search'"></span>
              </button>
            </div>
          </div>

          <!-- Order Selection -->
          <div class="order-selection-card">
            <h3 class="section-title" v-text="'Select Order'"></h3>
            <p class="section-subtitle" v-text="`Total orders: ${customerOrders.length}`"></p>

            <div v-if="customerOrders.length > 0" class="order-dropdown-container">
              <select v-model="selectedOrderId" class="order-dropdown" @change="loadOrderDetails">
                <option value="" v-text="'Select an order...'"></option>
                <option
                    v-for="order in customerOrders"
                    :key="order.id"
                    :value="order.id"
                    v-text="`${order.orderNumber} - ${formatDate(order.createdDate)}`"
                ></option>
              </select>
            </div>

            <div v-else class="no-orders-message">
              <i class="fa fa-inbox"></i>
              <p v-text="'No orders found for this customer'"></p>
            </div>
          </div>
        </div>

        <!-- Add Incident Section -->
        <div v-if="selectedCustomer && selectedOrderId" class="incident-form-section">
          <div class="form-card">
            <h2 class="section-title" v-text="'Add Incident Record'"></h2>
            <p class="section-subtitle" v-text="'Report an incident for this order'"></p>

            <form @submit.prevent="saveIncident">
              <!-- Incident Type -->
              <div class="form-group">
                <label for="incidentType" v-text="'Incident Type'"></label>
                <span class="required" v-text="'*'"></span>
                <select
                    id="incidentType"
                    v-model="incidentForm.type"
                    class="input-field select-field"
                    required
                >
                  <option value="" v-text="'Select type'"></option>
                  <option value="ACCIDENT" v-text="'Accident'"></option>
                  <option value="DAMAGE" v-text="'Damage'"></option>
                  <option value="LATE_FEE" v-text="'Late Fee'"></option>
                  <option value="FUEL_ISSUE" v-text="'Fuel Issue'"></option>
                  <option value="MECHANICAL" v-text="'Mechanical'"></option>
                  <option value="CLEANING" v-text="'Cleaning'"></option>
                  <option value="OTHER" v-text="'Other'"></option>
                </select>
              </div>

              <!-- Severity Level -->
              <div class="form-group">
                <label for="severityLevel" v-text="'Severity Level'"></label>
                <span class="required" v-text="'*'"></span>
                <select
                    id="severityLevel"
                    v-model="incidentForm.severity"
                    class="input-field select-field"
                    required
                >
                  <option value="" v-text="'Select level'"></option>
                  <option value="LOW" v-text="'Low'"></option>
                  <option value="MEDIUM" v-text="'Medium'"></option>
                  <option value="HIGH" v-text="'High'"></option>
                  <option value="CRITICAL" v-text="'Critical'"></option>
                </select>
              </div>

              <!-- Description -->
              <div class="form-group">
                <label for="incidentDescription" v-text="'Description'"></label>
                <span class="required" v-text="'*'"></span>
                <textarea
                    id="incidentDescription"
                    v-model="incidentForm.description"
                    class="input-field textarea"
                    placeholder="Describe the incident in detail..."
                    required
                ></textarea>
              </div>

              <!-- Estimated Cost -->
              <div class="form-group">
                <label for="estimatedCost" v-text="'Estimated Cost (LKR)'"></label>
                <input
                    id="estimatedCost"
                    v-model.number="incidentForm.estimatedCost"
                    type="number"
                    class="input-field"
                    placeholder="0.00"
                    min="0"
                    step="0.01"
                />
              </div>

              <!-- Incident Date -->
              <div class="form-group">
                <label for="incidentDate" v-text="'Incident Date'"></label>
                <span class="required" v-text="'*'"></span>
                <input
                    id="incidentDate"
                    v-model="incidentForm.date"
                    type="date"
                    class="input-field"
                    required
                />
              </div>

              <!-- Form Actions -->
              <div class="form-actions">
                <button type="button" class="btn-secondary" @click="clearForm">
                  <i class="fa fa-times"></i>
                  <span v-text="'Clear'"></span>
                </button>
                <button type="submit" class="btn-primary">
                  <i class="fa fa-save"></i>
                  <span v-text="'Record Incident'"></span>
                </button>
              </div>
            </form>
          </div>
        </div>

        <!-- Order Details Modal -->
        <div v-if="showOrderModal && selectedOrder" class="modal-overlay" @click="closeOrderModal">
          <div class="modal-content large" @click.stop>
            <div class="modal-header">
              <div>
                <h2 v-text="`Order ${selectedOrder.orderNumber}`"></h2>
                <p class="modal-subtitle" v-text="'Order Details'"></p>
              </div>
              <button class="btn-close" @click="closeOrderModal">
                <i class="fa fa-times"></i>
              </button>
            </div>

            <div class="modal-body">
              <div class="modal-grid">
                <!-- Order Info -->
                <div class="modal-section">
                  <h3 class="modal-section-title" v-text="'Order Information'"></h3>
                  <div class="info-row">
                    <span class="info-label" v-text="'Order Number:'"></span>
                    <span class="info-value" v-text="selectedOrder.orderNumber"></span>
                  </div>
                  <div class="info-row">
                    <span class="info-label" v-text="'Status:'"></span>
                    <span class="status-badge" :class="`status-${selectedOrder.status?.toLowerCase()}`">
                      {{ selectedOrder.status }}
                    </span>
                  </div>
                  <div class="info-row">
                    <span class="info-label" v-text="'Total Amount:'"></span>
                    <span class="info-value amount" v-text="formatPrice(selectedOrder.totalAmount)"></span>
                  </div>
                  <div class="info-row">
                    <span class="info-label" v-text="'Date:'"></span>
                    <span class="info-value" v-text="formatDate(selectedOrder.createdDate)"></span>
                  </div>
                </div>

                <!-- Customer Info -->
                <div class="modal-section">
                  <h3 class="modal-section-title" v-text="'Customer Information'"></h3>
                  <div class="info-row">
                    <span class="info-label" v-text="'Name:'"></span>
                    <span class="info-value" v-text="selectedOrder.customerName"></span>
                  </div>
                  <div class="info-row">
                    <span class="info-label" v-text="'National ID:'"></span>
                    <span class="info-value" v-text="selectedOrder.customerId"></span>
                  </div>
                  <div class="info-row">
                    <span class="info-label" v-text="'Contact:'"></span>
                    <span class="info-value" v-text="selectedOrder.contactNumber || '-'"></span>
                  </div>
                  <div class="info-row">
                    <span class="info-label" v-text="'License Number:'"></span>
                    <span class="info-value" v-text="selectedOrder.licenseNumber || '-'"></span>
                  </div>
                </div>

                <!-- Vehicle Info -->
                <div class="modal-section">
                  <h3 class="modal-section-title" v-text="'Vehicle Information'"></h3>
                  <div class="info-row">
                    <span class="info-label" v-text="'Vehicle Name:'"></span>
                    <span class="info-value" v-text="selectedOrder.vehicleName"></span>
                  </div>
                  <div class="info-row">
                    <span class="info-label" v-text="'Registration Number:'"></span>
                    <span class="info-value" v-text="selectedOrder.vehicleNumber"></span>
                  </div>
                  <div class="info-row">
                    <span class="info-label" v-text="'Start Mileage:'"></span>
                    <span class="info-value" v-text="`${selectedOrder.startMileage} km`"></span>
                  </div>
                  <div class="info-row">
                    <span class="info-label" v-text="'Rental Period:'"></span>
                    <span class="info-value" v-text="`${formatDate(selectedOrder.startDate)} to ${formatDate(selectedOrder.endDate)}`"></span>
                  </div>
                </div>
              </div>

              <!-- Images Section -->
              <div class="images-grid">
                <div class="image-section">
                  <h4 class="image-title" v-text="'Customer Photo'"></h4>
                  <div v-if="customerImageWithVehicle" class="image-container">
                    <img :src="customerImageWithVehicle" alt="Customer" class="detail-image">
                  </div>
                  <div v-else class="no-image">
                    <i class="fa fa-image"></i>
                    <p v-text="'No image'"></p>
                  </div>
                </div>
              </div>
            </div>

            <div class="modal-footer">
              <button class="btn-secondary" @click="closeOrderModal">
                <i class="fa fa-times"></i>
                <span v-text="'Close'"></span>
              </button>
            </div>
          </div>
        </div>

        <!-- Past Incidents Table -->
        <div v-if="selectedCustomer" class="incidents-table-section">
          <div class="table-section">
            <div class="table-header">
              <div>
                <h2 class="section-title" v-text="'Incident History'"></h2>
                <p class="table-subtitle" v-text="`Total incidents: ${customerIncidents.length}`"></p>
              </div>
            </div>

            <div class="table-wrapper">
              <table v-if="customerIncidents.length > 0" class="incidents-table">
                <thead>
                <tr>
                  <th v-text="'Date'"></th>
                  <th v-text="'Order ID'"></th>
                  <th v-text="'Type'"></th>
                  <th v-text="'Severity'"></th>
                  <th v-text="'Description'"></th>
                  <th v-text="'Cost (LKR)'"></th>
                  <th v-text="'Status'"></th>
                  <th v-text="'Actions'"></th>
                </tr>
                </thead>
                <tbody>
                <tr v-for="incident in customerIncidents" :key="incident.id">
                  <td v-text="formatDate(incident.date)"></td>
                  <td v-text="incident.orderId"></td>
                  <td>
                    <span class="type-badge" v-text="formatLabel(incident.type)"></span>
                  </td>
                  <td>
                    <span
                        class="severity-badge"
                        :style="{ backgroundColor: getSeverityColor(incident.severity) + '20', color: getSeverityColor(incident.severity) }"
                        v-text="formatLabel(incident.severity)"
                    ></span>
                  </td>
                  <td class="description-cell" :title="incident.description" v-text="incident.description.substring(0, 40) + (incident.description.length > 40 ? '...' : '')"></td>
                  <td class="amount-cell" v-text="formatPrice(incident.estimatedCost)"></td>
                  <td>
                    <span
                        class="incident-status-badge"
                        :class="`incident-status-${incident.status?.toLowerCase()}`"
                        v-text="formatLabel(incident.status)"
                    ></span>
                  </td>
                  <td class="actions">
                    <button
                        class="btn-action edit"
                        @click="editIncident(incident)"
                        title="Edit"
                    >
                      <i class="fa fa-edit"></i>
                    </button>
                    <button
                        class="btn-action delete"
                        @click="deleteIncident(incident.id)"
                        title="Delete"
                    >
                      <i class="fa fa-trash"></i>
                    </button>
                  </td>
                </tr>
                </tbody>
              </table>

              <div v-else class="empty-table-state">
                <i class="fa fa-inbox"></i>
                <p v-text="'No incident records found for this customer'"></p>
              </div>
            </div>
          </div>
        </div>

        <!-- Empty State -->
        <div v-if="!selectedCustomer" class="empty-state">
          <i class="fa fa-search"></i>
          <p v-text="'Search for a customer to view orders and manage incidents'"></p>
        </div>

      </div>
    </div>

    <!-- Blacklist Modal -->
    <div v-if="showBlacklistModal" class="modal-overlay" @click="closeBlacklistModal">
      <div class="modal-content" @click.stop>
        <div class="modal-header danger">
          <h2 v-text="'Add to Blacklist'"></h2>
          <button class="btn-close" @click="closeBlacklistModal" title="Close">
            <i class="fa fa-times"></i>
          </button>
        </div>

        <div class="modal-body">
          <form @submit.prevent="saveBlacklist">
            <!-- Customer Info Display -->
            <div class="customer-info-display">
              <p class="info-label" v-text="'Customer ID'"></p>
              <p class="info-display" v-text="selectedCustomer?.customerId"></p>
            </div>

            <!-- Blacklist Reason -->
            <div class="form-group">
              <label for="blacklistReason" v-text="'Reason for Blacklisting'"></label>
              <span class="required" v-text="'*'"></span>
              <select
                  id="blacklistReason"
                  v-model="blacklistForm.reason"
                  class="input-field select-field"
                  required
              >
                <option value="" v-text="'Select reason'"></option>
                <option value="MULTIPLE_ACCIDENTS" v-text="'Multiple Accidents'"></option>
                <option value="UNPAID_FEES" v-text="'Unpaid Fees'"></option>
                <option value="PROPERTY_DAMAGE" v-text="'Property Damage'"></option>
                <option value="RECKLESS_BEHAVIOR" v-text="'Reckless Behavior'"></option>
                <option value="FRAUD_SUSPECTED" v-text="'Fraud Suspected'"></option>
                <option value="OTHER" v-text="'Other'"></option>
              </select>
            </div>

            <!-- Blacklist Notes -->
            <div class="form-group">
              <label for="blacklistNotes" v-text="'Additional Notes'"></label>
              <textarea
                  id="blacklistNotes"
                  v-model="blacklistForm.notes"
                  class="input-field textarea"
                  placeholder="Add any additional details..."
                  rows="4"
              ></textarea>
            </div>

            <!-- Blacklist Date -->
            <div class="form-group">
              <label for="blacklistDate" v-text="'Date'"></label>
              <input
                  id="blacklistDate"
                  v-model="blacklistForm.date"
                  type="date"
                  class="input-field"
                  required
              />
            </div>

            <!-- Form Actions -->
            <div class="form-actions">
              <button type="button" class="btn-secondary" @click="closeBlacklistModal">
                <i class="fa fa-times"></i>
                <span v-text="'Cancel'"></span>
              </button>
              <button type="submit" class="btn-danger">
                <i class="fa fa-ban"></i>
                <span v-text="'Add to Blacklist'"></span>
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import HeaderComponent from '../component/Header.vue';
import { dbService } from '../services/db.ts';
import { useAuthStore } from "../stores/auth.ts";
import {useSnackbar} from "../composables/useSnackbar.js";
const { showSuccess, showError } = useSnackbar();

export default {
  name: 'IncidentManagement',
  components: {
    HeaderComponent
  },

  data() {
    return {
      loggedUser: null,
      roleName: null,
      searchCustomerId: '',
      searchError: '',
      selectedCustomer: null,
      selectedOrder: null,
      selectedOrderId: '',
      customerOrders: [],
      customerIncidents: [],
      showOrderModal: false,
      showBlacklistModal: false,
      incidentForm: {
        type: '',
        severity: '',
        description: '',
        estimatedCost: 0,
        date: new Date().toISOString().slice(0, 10)
      },
      blacklistForm: {
        reason: '',
        notes: '',
        date: new Date().toISOString().slice(0, 10)
      },
      customerImageWithVehicle: null,
    };
  },
  created() {
    const authStore = useAuthStore()
    this.loggedUser = authStore.username
    this.roleName = authStore.role
  },
  methods: {
    formatLabel(value) {
      if (!value) return '';
      return value
          .split('_')
          .map(word => word.charAt(0).toUpperCase() + word.slice(1).toLowerCase())
          .join(' ');
    },

    formatPrice(price) {
      return new Intl.NumberFormat('en-US', {
        style: 'currency',
        currency: 'LKR',
        minimumFractionDigits: 2,
        maximumFractionDigits: 2
      }).format(Number(price || 0));
    },

    formatDate(dateString) {
      if (!dateString) return '-';
      return new Date(dateString).toLocaleDateString('en-US', {
        year: 'numeric',
        month: 'short',
        day: 'numeric'
      });
    },

    getSeverityColor(severity) {
      const colors = {
        LOW: '#22c55e',
        MEDIUM: '#f59e0b',
        HIGH: '#ef4444',
        CRITICAL: '#7c2d12'
      };
      return colors[severity] || '#666666';
    },

    async searchCustomer() {
      if (!this.searchCustomerId.trim()) {
        this.searchError = 'Please enter a customer ID';
        return;
      }

      try {
        this.searchError = '';
        const customer = await dbService.getCustomerByIdentity(this.searchCustomerId);
        if (customer.length > 0) {
          this.selectedCustomer = {
            customerId: customer[0].customer_id,
            customerName: customer[0].customer_name,
            contactNumber: customer[0].contact_no,
            licenseNumber: customer[0].license_number,
            isBlacklisted: customer[0].is_blacked_listed || false
          };

          await this.loadCustomerOrders();
          await this.loadCustomerIncidents();
        } else {
          this.searchError = 'Customer not found';
          this.selectedCustomer = null;
        }
      } catch (error) {
        console.error('Error searching customer:', error);
        this.searchError = 'Error searching customer. Please try again.';
      }
    },

    async loadCustomerOrders() {
      try {
        const orders = await dbService.getCustomerOrders(this.selectedCustomer.customerId);
        this.customerOrders = orders.map(order => ({
          id: order.order_number,
          orderNumber: order.order_number,
          createdDate: order.created_at,
          status: order.order_status
        }));
        this.selectedOrderId = '';
      } catch (error) {
        console.error('Error loading orders:', error);
        this.customerOrders = [];
      }
    },

    async loadCustomerIncidents() {
      try {
        const incidents = await dbService.getIncidents(this.selectedCustomer.customerId);
        this.customerIncidents = incidents.map(incident => ({
          id: incident.id,
          orderId: incident.order_number,
          type: incident.incident_type,
          severity: incident.severity,
          description: incident.description,
          estimatedCost: Number(incident.estimation_cost || 0),
          date: incident.incident_date,
          status: incident.status
        }));
      } catch (error) {
        console.error('Error loading incidents:', error);
        this.customerIncidents = [];
      }
    },

    async loadOrderDetails() {
      if (!this.selectedOrderId) {
        this.selectedOrder = null;
        return;
      }

      try {
        const order = await dbService.getOrderByOrderNumber(this.selectedOrderId);

        this.selectedOrder = {
          id: order[0].id,
          orderNumber: order[0].order_number,
          customerId: order[0].customer_id,
          customerName: order[0].customer_name,
          contactNumber: order[0].contact_no,
          licenseNumber: order[0].license_number,
          manufacturer: order[0].manufacturer,
          modelName: order[0].model_name,
          vehicleNumber: order[0].vehicle_register_number,
          startMileage: order[0].starting_mileage,
          startDate: order[0].release_time,
          endDate: order[0].handover_time,
          totalAmount: order[0].total_amount,
          status: order[0].order_status,
          createdDate: order[0].created_at,
        };

        if (order[0].customer_image_with_vehicle) {
          const bytes = new Uint8Array(order[0].customer_image_with_vehicle)
          const blob = new Blob(
              [bytes],
              { type: 'image/jpg' }
          )
          this.customerImageWithVehicle = URL.createObjectURL(blob)
        }
        this.showOrderModal = true;
      } catch (error) {
        console.error('Error loading order details:', error);
        alert('Error loading order details');
      }
    },

    closeOrderModal() {
      this.showOrderModal = false;
    },

    async saveIncident() {
      if (
          !this.incidentForm.type ||
          !this.incidentForm.severity ||
          !this.incidentForm.description ||
          !this.incidentForm.date ||
          !this.selectedCustomer ||
          !this.selectedOrderId
      ) {
        alert('Please fill all required fields');
        return;
      }

      try {
        const record = {
          order_number: this.selectedOrderId,
          customer_id: this.selectedCustomer.customerId,
          incident_type: this.incidentForm.type,
          severity: this.incidentForm.severity,
          description: this.incidentForm.description,
          estimation_cost: this.incidentForm.estimatedCost,
          incident_date: this.incidentForm.date,
          status: 'OPEN',
          created_by: 'ADMIN'
        };

        await dbService.addIncident(record);
        alert('Incident recorded successfully!');
        this.clearForm();
        await this.loadCustomerIncidents();
      } catch (error) {
        console.error('Error saving incident:', error);
        alert('Error recording incident. Please try again.');
      }
    },

    clearForm() {
      this.incidentForm = {
        type: '',
        severity: '',
        description: '',
        estimatedCost: 0,
        date: new Date().toISOString().slice(0, 10)
      };
    },

    async deleteIncident(id) {
      if (!confirm('Are you sure you want to delete this incident record?')) return;

      try {
        await dbService.deleteIncident(id);
        const index = this.customerIncidents.findIndex(item => item.id === id);
        if (index !== -1) {
          this.customerIncidents.splice(index, 1);
        }
        alert('Incident deleted successfully!');
      } catch (error) {
        console.error('Error deleting incident:', error);
        alert('Error deleting incident. Please try again.');
      }
    },

    editIncident(incident) {
      this.incidentForm = {
        type: incident.type,
        severity: incident.severity,
        description: incident.description,
        estimatedCost: incident.estimatedCost,
        date: incident.date
      };
      window.scrollTo({ top: 0, behavior: 'smooth' });
    },

    openBlacklistModal() {
      if (!this.selectedCustomer) return;
      this.blacklistForm = {
        reason: '',
        notes: '',
        date: new Date().toISOString().slice(0, 10)
      };
      this.showBlacklistModal = true;
    },

    closeBlacklistModal() {
      this.showBlacklistModal = false;
    },

    async saveBlacklist() {
      if (!this.blacklistForm.reason) {
        alert('Please select a reason for blacklisting');
        return;
      }

      try {
        const record = {
          customer_id: this.selectedCustomer.customerId,
          reason: this.blacklistForm.reason,
          notes: this.blacklistForm.notes,
          black_listed_date: this.blacklistForm.date,
          created_by: this.loggedUser
        };

        await dbService.addRestrictedCustomers(record);
        showSuccess('Customer added to blacklist successfully!');
        this.selectedCustomer.isBlacklisted = true;
        this.closeBlacklistModal();
      } catch (error) {
        console.error('Error blacklisting customer:', error);
        showError('Error adding to blacklist. Please try again.');
      }
    },

    async removeFromBlacklist() {
      try {
        await dbService.removeRestrictedCustomer(this.selectedCustomer.customerId);
        showSuccess('Customer removed from blacklist successfully!');
        this.selectedCustomer.isBlacklisted = false;
      } catch (error) {
        console.error('Error removing from blacklist:', error);
        showError('Error removing from blacklist. Please try again.');
      }
    },

    newSearch() {
      this.selectedCustomer = null;
      this.selectedOrder = null;
      this.selectedOrderId = '';
      this.customerOrders = [];
      this.customerIncidents = [];
      this.searchCustomerId = '';
      this.searchError = '';
      this.clearForm();
    },

    handleLogout() {
      this.$emit('logout');
    }
  }
};
</script>

<style scoped>
.incident-page {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
  background: var(--color-background-tertiary);
}

.incident-content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.incident-container {
  flex: 1;
  overflow-y: auto;
  padding: 1.5rem;
  max-width: 1400px;
  margin: 0 auto;
  width: 100%;
}

/* === PAGE HEADER === */
.page-header {
  margin-bottom: 2rem;
}

.page-header h1 {
  margin: 0;
  font-size: 28px;
  font-weight: 700;
  color: var(--color-text-primary);
}

.page-subtitle {
  margin: 0.5rem 0 0;
  font-size: 13px;
  color: var(--color-text-secondary);
  font-weight: 400;
}

/* === SEARCH SECTION === */
.search-section {
  margin-bottom: 2rem;
}

.search-card {
  background: var(--color-background-primary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-lg);
  padding: 1.5rem;
}

.section-title {
  margin: 0 0 0.5rem;
  font-size: 16px;
  font-weight: 700;
  color: var(--color-text-primary);
}

.section-subtitle {
  margin: 0 0 1.5rem;
  font-size: 12px;
  color: var(--color-text-secondary);
}

.search-container {
  display: flex;
  gap: 0.75rem;
}

.search-input-large {
  flex: 1;
  padding: 0.85rem 1rem;
  border: 1.5px solid var(--color-border-tertiary);
  border-radius: 6px;
  font-size: 14px;
  color: var(--color-text-primary);
  background: var(--color-background-primary);
  transition: all var(--transition-fast);
}

.search-input-large:focus {
  outline: none;
  border-color: var(--color-info);
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}

.btn-search {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.85rem 1.5rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-weight: 600;
  font-size: 14px;
  transition: all var(--transition-normal);
  white-space: nowrap;
}

.btn-search:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
}

.error-message {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin-top: 1rem;
  padding: 0.85rem 1rem;
  background: rgba(239, 68, 68, 0.1);
  border: 1px solid rgba(239, 68, 68, 0.3);
  border-radius: 6px;
  color: #dc2626;
  font-size: 13px;
}

.error-message i {
  font-size: 16px;
}

/* === CUSTOMER SECTION === */
.customer-section {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1.5rem;
  margin-bottom: 2rem;
}

.customer-info-card {
  background: var(--color-background-primary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-lg);
  padding: 1.5rem;
}

.customer-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 1.5rem;
  padding-bottom: 1rem;
  border-bottom: 1px solid var(--color-border-tertiary);
}

.customer-name {
  margin: 0;
  font-size: 18px;
  font-weight: 700;
  color: var(--color-text-primary);
}

.customer-id {
  margin: 0.3rem 0 0;
  font-size: 12px;
  color: var(--color-text-secondary);
}

.customer-status {
  display: flex;
  gap: 0.5rem;
}

.blacklist-badge,
.active-badge {
  display: flex;
  align-items: center;
  gap: 0.4rem;
  padding: 0.4rem 0.75rem;
  border-radius: 6px;
  font-size: 12px;
  font-weight: 600;
  white-space: nowrap;
}

.blacklist-badge {
  background: rgba(239, 68, 68, 0.15);
  color: #dc2626;
}

.active-badge {
  background: rgba(34, 197, 94, 0.15);
  color: #15803d;
}

.customer-details {
  margin-bottom: 1.5rem;
}

.detail-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem 0;
  font-size: 13px;
}

.detail-label {
  font-weight: 600;
  color: var(--color-text-secondary);
}

.detail-value {
  color: var(--color-text-primary);
}

.customer-actions {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.btn-blacklist,
.btn-remove-blacklist,
.btn-new-search {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  padding: 0.75rem 1rem;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-weight: 600;
  font-size: 13px;
  transition: all var(--transition-normal);
}

.btn-blacklist {
  background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
  color: white;
}

.btn-blacklist:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(239, 68, 68, 0.3);
}

.btn-remove-blacklist {
  background: linear-gradient(135deg, #22c55e 0%, #16a34a 100%);
  color: white;
}

.btn-remove-blacklist:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(34, 197, 94, 0.3);
}

.btn-new-search {
  background: transparent;
  color: var(--color-text-primary);
  border: 1px solid var(--color-border-tertiary);
}

.btn-new-search:hover {
  background: var(--color-background-secondary);
  border-color: var(--color-info);
  color: var(--color-info);
}

/* === ORDER SELECTION === */
.order-selection-card {
  background: var(--color-background-primary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-lg);
  padding: 1.5rem;
}

.order-dropdown-container {
  margin-bottom: 1rem;
}

.order-dropdown {
  width: 100%;
  padding: 0.85rem 1rem;
  border: 1.5px solid var(--color-border-tertiary);
  border-radius: 6px;
  font-size: 14px;
  color: var(--color-text-primary);
  background: var(--color-background-primary);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.order-dropdown:hover {
  border-color: var(--color-border-secondary);
}

.order-dropdown:focus {
  outline: none;
  border-color: var(--color-info);
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}

.no-orders-message {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 2rem;
  color: var(--color-text-secondary);
  text-align: center;
}

.no-orders-message i {
  font-size: 32px;
  margin-bottom: 0.75rem;
  opacity: 0.3;
}

/* === INCIDENT FORM SECTION === */
.incident-form-section {
  margin-bottom: 2rem;
}

.form-card {
  background: var(--color-background-primary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-lg);
  padding: 1.5rem;
}

.form-group {
  margin-bottom: 1.5rem;
}

.form-group label {
  display: flex;
  align-items: center;
  gap: 0.3rem;
  margin-bottom: 0.6rem;
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.required {
  color: #ef4444;
  font-weight: 700;
}

.input-field {
  width: 100%;
  box-sizing: border-box;
  padding: 0.8rem 0.9rem;
  border: 1.5px solid var(--color-border-tertiary);
  border-radius: 6px;
  font-size: 14px;
  color: var(--color-text-primary);
  background: var(--color-background-primary);
  transition: all var(--transition-fast);
  font-family: inherit;
}

.input-field::placeholder {
  color: var(--color-text-secondary);
  opacity: 0.7;
}

.input-field:hover {
  border-color: var(--color-border-secondary);
}

.input-field:focus {
  outline: none;
  border-color: var(--color-info);
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}

.select-field {
  appearance: none;
  background-image: url("data:image/svg+xml;charset=UTF-8,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='%23667eea' stroke-width='2.5' stroke-linecap='round' stroke-linejoin='round'%3e%3cpolyline points='6 9 12 15 18 9'%3e%3c/polyline%3e%3c/svg%3e");
  background-repeat: no-repeat;
  background-position: right 0.85rem center;
  background-size: 1.3rem;
  padding-right: 2.6rem;
  cursor: pointer;
}

.select-field:hover {
  border-color: var(--color-info);
  background-color: rgba(102, 126, 234, 0.03);
}

.select-field:focus {
  outline: none;
  border-color: var(--color-info);
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}

.textarea {
  resize: vertical;
  min-height: 100px;
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
  padding-top: 1.5rem;
  border-top: 1px solid var(--color-border-tertiary);
  margin-top: 1.5rem;
}

.btn-primary,
.btn-secondary,
.btn-danger {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.25rem;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-weight: 600;
  font-size: 14px;
  transition: all var(--transition-normal);
}

.btn-primary {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

.btn-primary:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
}

.btn-secondary {
  background: transparent;
  color: var(--color-text-primary);
  border: 1px solid var(--color-border-tertiary);
}

.btn-secondary:hover {
  background: var(--color-background-secondary);
  border-color: var(--color-text-secondary);
}

.btn-danger {
  background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
  color: white;
}

.btn-danger:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(239, 68, 68, 0.3);
}

/* === MODAL === */
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
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

.modal-content {
  background: var(--color-background-primary);
  border-radius: var(--border-radius-lg);
  width: 90%;
  max-width: 600px;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
  animation: slideUp 0.3s ease;
}

.modal-content.large {
  max-width: 900px;
  max-height: 85vh;
}

@keyframes slideUp {
  from {
    transform: translateY(20px);
    opacity: 0;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.25rem 1.5rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border-radius: var(--border-radius-lg) var(--border-radius-lg) 0 0;
}

.modal-header.danger {
  background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
}

.modal-header h2 {
  margin: 0;
  font-size: 19px;
  font-weight: 700;
}

.modal-subtitle {
  margin: 0.3rem 0 0;
  font-size: 12px;
  opacity: 0.85;
}

.btn-close {
  padding: 0.4rem;
  border: none;
  background: transparent;
  color: white;
  cursor: pointer;
  font-size: 20px;
  transition: color var(--transition-fast);
  display: flex;
  align-items: center;
  justify-content: center;
}

.btn-close:hover {
  color: rgba(255, 255, 255, 0.8);
}

.modal-body {
  padding: 1.5rem;
}

.modal-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1.5rem;
  margin-bottom: 2rem;
}

.modal-section {
  background: var(--color-background-secondary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: 8px;
  padding: 1rem;
}

.modal-section-title {
  margin: 0 0 1rem;
  font-size: 13px;
  font-weight: 700;
  color: var(--color-text-primary);
  padding-bottom: 0.75rem;
  border-bottom: 1px solid var(--color-border-tertiary);
}

.info-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.75rem;
  padding: 0.5rem 0;
  font-size: 12px;
}

.info-label {
  font-weight: 600;
  color: var(--color-text-secondary);
}

.info-value {
  color: var(--color-text-primary);
  text-align: right;
}

.info-value.amount {
  color: #667eea;
  font-family: monospace;
  font-weight: 700;
}

.status-badge {
  display: inline-block;
  padding: 0.35rem 0.75rem;
  border-radius: 6px;
  font-size: 11px;
  font-weight: 600;
}

.status-badge.status-active {
  background: rgba(34, 197, 94, 0.15);
  color: #15803d;
}

.status-badge.status-completed {
  background: rgba(34, 197, 94, 0.15);
  color: #15803d;
}

.status-badge.status-cancelled {
  background: rgba(239, 68, 68, 0.15);
  color: #dc2626;
}

.images-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.image-section {
  background: var(--color-background-secondary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: 8px;
  padding: 1rem;
  overflow: hidden;
}

.image-title {
  margin: 0 0 0.75rem;
  font-size: 12px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.image-container {
  width: 100%;
  height: 200px;
  border-radius: 6px;
  overflow: hidden;
  background: var(--color-background-tertiary);
  display: flex;
  align-items: center;
  justify-content: center;
}

.detail-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  border-radius: 6px;
}

.no-image {
  width: 100%;
  height: 200px;
  border-radius: 6px;
  background: var(--color-background-tertiary);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: var(--color-text-secondary);
}

.no-image i {
  font-size: 32px;
  margin-bottom: 0.5rem;
  opacity: 0.3;
}

.no-image p {
  margin: 0;
  font-size: 11px;
}

.modal-footer {
  padding: 1rem 1.5rem;
  border-top: 1px solid var(--color-border-tertiary);
  text-align: right;
}

.customer-info-display {
  background: var(--color-background-secondary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: 6px;
  padding: 1rem;
  margin-bottom: 1.5rem;
}

.customer-info-display .info-label {
  font-size: 12px;
  font-weight: 600;
  color: var(--color-text-secondary);
  margin: 0 0 0.5rem;
}

.customer-info-display .info-display {
  font-size: 15px;
  font-weight: 700;
  color: var(--color-text-primary);
  margin: 0;
  font-family: monospace;
}

/* === INCIDENTS TABLE === */
.incidents-table-section {
  margin-bottom: 2rem;
}

.table-section {
  background: var(--color-background-primary);
  border-radius: var(--border-radius-lg);
  border: 1px solid var(--color-border-tertiary);
  overflow: hidden;
}

.table-header {
  padding: 1.25rem 1.5rem;
  border-bottom: 1px solid var(--color-border-tertiary);
}

.table-subtitle {
  margin: 0.5rem 0 0;
  font-size: 12px;
  color: var(--color-text-secondary);
}

.table-wrapper {
  overflow-x: auto;
}

.incidents-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

.incidents-table th {
  padding: 0.9rem 0.8rem;
  text-align: left;
  color: var(--color-text-secondary);
  background: var(--color-background-secondary);
  font-weight: 600;
  text-transform: capitalize;
}

.incidents-table td {
  padding: 1rem 0.8rem;
  border-bottom: 1px solid var(--color-border-tertiary);
  color: var(--color-text-primary);
  vertical-align: middle;
}

.incidents-table tbody tr:hover {
  background: var(--color-background-secondary);
}

.type-badge {
  display: inline-block;
  padding: 0.4rem 0.75rem;
  border-radius: 6px;
  background: #dbeafe;
  color: #1e40af;
  font-size: 12px;
  font-weight: 600;
  white-space: nowrap;
}

.severity-badge {
  display: inline-block;
  padding: 0.4rem 0.75rem;
  border-radius: 6px;
  font-size: 12px;
  font-weight: 600;
  white-space: nowrap;
}

.incident-status-badge {
  display: inline-block;
  padding: 0.4rem 0.75rem;
  border-radius: 6px;
  font-size: 12px;
  font-weight: 600;
  white-space: nowrap;
}

.incident-status-badge.incident-status-open {
  background: rgba(59, 130, 246, 0.15);
  color: #1e40af;
}

.incident-status-badge.incident-status-in_progress {
  background: rgba(245, 158, 11, 0.15);
  color: #b45309;
}

.incident-status-badge.incident-status-resolved {
  background: rgba(34, 197, 94, 0.15);
  color: #15803d;
}

.incident-status-badge.incident-status-closed {
  background: rgba(107, 114, 128, 0.15);
  color: #374151;
}

.description-cell {
  max-width: 300px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.amount-cell {
  font-family: monospace;
  font-weight: 700;
  color: #ef4444;
  text-align: right;
}

.actions {
  display: flex;
  gap: 0.4rem;
  justify-content: center;
}

.btn-action {
  padding: 0.4rem 0.5rem;
  border: 1px solid var(--color-border-tertiary);
  background: transparent;
  color: var(--color-text-secondary);
  border-radius: 4px;
  cursor: pointer;
  transition: all var(--transition-fast);
  font-size: 14px;
}

.btn-action.edit:hover {
  color: #f59e0b;
  border-color: #f59e0b;
  background: rgba(245, 158, 11, 0.05);
}

.btn-action.delete:hover {
  color: #ef4444;
  border-color: #ef4444;
  background: rgba(239, 68, 68, 0.05);
}

/* === EMPTY STATE === */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 4rem 2rem;
  color: var(--color-text-secondary);
}

.empty-state i {
  font-size: 64px;
  margin-bottom: 1rem;
  opacity: 0.2;
}

.empty-state p {
  margin: 0;
  font-size: 14px;
}

.empty-table-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 3rem 2rem;
  color: var(--color-text-secondary);
}

.empty-table-state i {
  font-size: 48px;
  margin-bottom: 0.75rem;
  opacity: 0.2;
}

.empty-table-state p {
  margin: 0;
  font-size: 13px;
}

/* === RESPONSIVE === */
@media (max-width: 1024px) {
  .customer-section {
    grid-template-columns: 1fr;
  }

  .modal-grid {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 768px) {
  .incident-container {
    padding: 1rem;
  }

  .page-header h1 {
    font-size: 24px;
  }

  .customer-header {
    flex-direction: column;
    align-items: flex-start;
  }

  .customer-actions {
    flex-direction: row;
    flex-wrap: wrap;
  }

  .customer-actions button {
    flex: 1;
    min-width: 120px;
  }

  .search-container {
    flex-direction: column;
  }

  .btn-search {
    width: 100%;
  }

  .form-actions {
    flex-direction: column;
  }

  .form-actions button {
    width: 100%;
  }

  .modal-content {
    width: 95%;
  }

  .modal-grid {
    grid-template-columns: 1fr;
  }

  .images-grid {
    grid-template-columns: 1fr;
  }
}

/* Scrollbar styling */
.incident-container::-webkit-scrollbar,
.table-wrapper::-webkit-scrollbar,
.modal-content::-webkit-scrollbar {
  width: 6px;
}

.incident-container::-webkit-scrollbar-track,
.table-wrapper::-webkit-scrollbar-track,
.modal-content::-webkit-scrollbar-track {
  background: transparent;
}

.incident-container::-webkit-scrollbar-thumb,
.table-wrapper::-webkit-scrollbar-thumb,
.modal-content::-webkit-scrollbar-thumb {
  background: var(--color-border-secondary);
  border-radius: 3px;
}

.incident-container::-webkit-scrollbar-thumb:hover,
.table-wrapper::-webkit-scrollbar-thumb:hover,
.modal-content::-webkit-scrollbar-thumb:hover {
  background: var(--color-border-primary);
}
</style>
