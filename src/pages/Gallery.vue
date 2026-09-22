<template>
  <div class="dashboard-page">
    <!-- Header -->
    <header-component
        :showBackButton="true"
        @logout="handleLogout"
    />

    <!-- Dashboard Content -->
    <div class="dashboard-content">
      <div class="dashboard-container">

        <!-- Page Header -->
        <div class="page-header">
          <div>
            <h1>Customer & Vehicle Verification</h1>
            <p>View customer photos with vehicle details for check-in and check-out</p>
          </div>
        </div>

        <!-- Search Section with Parameters -->
        <div class="section">
          <div class="section-header">
            <h2>{{ hasSearched ? 'Search Results' : 'Today\'s Reservations' }}</h2>
            <p v-if="!hasSearched">Showing orders for today</p>
            <p v-else>Found {{ displayedOrders.length }} order{{ displayedOrders.length !== 1 ? 's' : '' }}</p>
          </div>

          <!-- Search Bar with Parameter Selection -->
          <div class="search-wrapper">
            <div class="search-parameter-group">
              <label for="search-param" class="search-param-label">Search by:</label>
              <select
                  v-model="searchParameter"
                  id="search-param"
                  class="search-parameter-select"
              >
                <option value="customerId">Customer ID</option>
                <option value="orderId">Order ID</option>
                <option value="vehicleId">Vehicle ID</option>
                <option value="date">Date</option>
              </select>
            </div>

            <div class="search-input-group">
              <i class="fa fa-search"></i>
              <input
                  v-model="searchQuery"
                  :type="searchParameter === 'date' ? 'date' : 'text'"
                  :placeholder="getSearchPlaceholder()"
                  class="search-input"
                  @keyup.enter="performSearch"
              />
            </div>
            <button @click="performSearch" class="search-button">
              <i class="fa fa-arrow-right"></i>
              Search
            </button>
            <button v-if="hasSearched" @click="clearSearch" class="clear-button">
              <i class="fa fa-times"></i>
              Clear
            </button>
          </div>

          <!-- Orders Grid - Display Today's Orders or Search Results -->
          <div class="cards-grid">
            <!-- Order Card -->
            <div class="order-card" v-for="order in displayedOrders" :key="order.id">
              <!-- Customer Photo -->
              <div class="customer-photo" @click="openImageModal(order.photo, order.customerName)">
                <div class="photo-container">
                  <img
                      :src="order.photo"
                      :alt="`${order.customerName}`"
                      loading="lazy"
                      class="card-image"
                  />
                  <div class="photo-overlay">
                    <i class="fa fa-search-plus"></i>
                  </div>
                </div>
              </div>

              <!-- Card Content -->
              <div class="card-content">
                <!-- Status Badge -->
                <div class="status-wrapper">
                  <div>
                    <p class="label">Customer ID</p>
                    <h3 class="customer-id">{{ order.customerId }}</h3>
                  </div>
                  <span :class="['status-badge', order.status.toLowerCase()]">
                    {{ order.status }}
                  </span>
                </div>

                <!-- Vehicle Info -->
                <div class="info-section">
                  <p class="label">Vehicle number</p>
                  <h4 class="vehicle-number">{{ order.vehicleNumber }}</h4>
                </div>

                <!-- Time Info -->
                <div class="time-info">
                  <div class="time-column">
                    <p class="label">Start time</p>
                    <p class="time-value">{{ formatTime(order.startTime) }}</p>
                  </div>
                  <div class="time-column">
                    <p class="label">End time</p>
                    <p class="time-value">{{ formatTime(order.endTime) }}</p>
                  </div>
                </div>
                <button class="view-button" @click="viewOrder(order.id)">
                  <i class="fa fa-arrow-right"></i>
                  <span>View Details</span>
                </button>
              </div>
            </div>
          </div>

          <!-- Empty State -->
          <div v-if="displayedOrders.length === 0" class="empty-state">
            <i :class="hasSearched ? 'fa fa-search' : 'fa fa-calendar-o'"></i>
            <p v-if="hasSearched">No orders found for "{{ searchQuery }}"</p>
            <p v-else>No reservations for today</p>
          </div>
        </div>

      </div>
    </div>

    <!-- Image Modal -->
    <transition name="modal-fade">
      <div v-if="imageModal.isOpen" class="image-modal-overlay" @click="closeImageModal">
        <div class="image-modal-content" @click.stop>
          <button class="modal-close-btn" @click="closeImageModal" title="Close">
            <i class="fa fa-times"></i>
          </button>
          <div class="modal-image-wrapper">
            <img
                :src="imageModal.imageSrc"
                :alt="imageModal.customerName"
                class="modal-image"
            />
          </div>
          <div class="modal-info">
            <h3>{{ imageModal.customerName }}</h3>
          </div>
        </div>
      </div>
    </transition>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import HeaderComponent from '../component/Header.vue';
import { dbService } from '../services/db.ts';

const router = useRouter();

// Reactive state
const searchQuery = ref('');
const searchParameter = ref('customerId');
const hasSearched = ref(false);
const searchResults = ref([]);
const todayOrders = ref([]);

// Computed property to display either today's orders or search results
const displayedOrders = computed(() => {
  return hasSearched.value ? searchResults.value : todayOrders.value;
});

// Modal state
const imageModal = ref({
  isOpen: false,
  imageSrc: '',
  customerName: ''
});

// Sample data - replace with API calls
const allOrders = [
  {
    id: 1,
    customerId: 'CUS-0045',
    customerName: 'John Doe',
    photo: 'https://via.placeholder.com/400x300?text=John+Doe',
    vehicleNumber: 'KL-7834',
    startTime: '09:00',
    endTime: '17:00',
    status: 'Active',
    date: '2024-01-15'
  },
  {
    id: 2,
    customerId: 'CUS-0068',
    customerName: 'Sarah Miller',
    photo: 'https://via.placeholder.com/400x300?text=Sarah+Miller',
    vehicleNumber: 'KL-5521',
    startTime: '10:30',
    endTime: '18:30',
    status: 'Pending',
    date: '2024-01-15'
  },
  {
    id: 3,
    customerId: 'CUS-0102',
    customerName: 'Alex Parker',
    photo: 'https://via.placeholder.com/400x300?text=Alex+Parker',
    vehicleNumber: 'KL-9012',
    startTime: '11:00',
    endTime: '19:00',
    status: 'Active',
    date: '2024-01-16'
  },
  {
    id: 4,
    customerId: 'CUS-0145',
    customerName: 'Mike Johnson',
    photo: 'https://via.placeholder.com/400x300?text=Mike+Johnson',
    vehicleNumber: 'KL-4456',
    startTime: '08:00',
    endTime: '16:00',
    status: 'Completed',
    date: '2024-01-14'
  },
  {
    id: 5,
    customerId: 'CUS-0178',
    customerName: 'Emma Wilson',
    photo: 'https://via.placeholder.com/400x300?text=Emma+Wilson',
    vehicleNumber: 'KL-5567',
    startTime: '13:00',
    endTime: '21:00',
    status: 'Pending',
    date: '2024-01-15'
  }
];

// Methods
const handleLogout = () => {
  router.push({ name: 'logout' });
};

const viewOrder = (orderId) => {
  router.push({
    name: 'view-order-detail',
    params: { id: orderId },
  });
};

const formatTime = (time) => {
  if (!time) return '';
  const [hours, minutes] = time.split(':');
  const hour = parseInt(hours, 10);
  const ampm = hour >= 12 ? 'PM' : 'AM';
  const displayHour = hour % 12 || 12;
  return `${displayHour.toString().padStart(2, '0')}:${minutes} ${ampm}`;
};

const getCustomerImage = (imageArray) => {
  if (imageArray && imageArray.length > 0) {
    const bytes = new Uint8Array(imageArray);
    const blob = new Blob([bytes], { type: 'image/jpg' });
    return URL.createObjectURL(blob);
  } else {
    return '/images/img.png';
  }
};

const getSearchPlaceholder = () => {
  const placeholders = {
    customerId: 'e.g., CUS-0045',
    orderId: 'e.g., 1, 2, 3',
    vehicleId: 'e.g., KL-7834',
    date: 'Select a date'
  };
  return placeholders[searchParameter.value] || 'Enter search term';
};

const performSearch = () => {
  if (searchQuery.value.trim() === '') {
    searchResults.value = [];
    hasSearched.value = false;
    return;
  }

  const query = searchQuery.value.toLowerCase();

  searchResults.value = allOrders.filter(order => {
    switch (searchParameter.value) {
      case 'customerId':
        return order.customerId.toLowerCase().includes(query);
      case 'orderId':
        return order.id.toString().includes(query);
      case 'vehicleId':
        return order.vehicleNumber.toLowerCase().includes(query);
      case 'date':
        return order.date === searchQuery.value;
      default:
        return false;
    }
  });

  hasSearched.value = true;
};

const clearSearch = () => {
  searchQuery.value = '';
  searchResults.value = [];
  hasSearched.value = false;
};

const openImageModal = (imageSrc, customerName) => {
  imageModal.value.isOpen = true;
  imageModal.value.imageSrc = imageSrc;
  imageModal.value.customerName = customerName;
  document.body.style.overflow = 'hidden';
};

const closeImageModal = () => {
  imageModal.value.isOpen = false;
  document.body.style.overflow = '';
};

const fetchingData = async () => {
  try {
    const filterOrders = await dbService.filterOrder('test');
    todayOrders.value = filterOrders.map(item => ({
      id: item.order_number,
      customerId: item.customer_id,
      customerName: item.customer_name || 'Unknown',
      photo: getCustomerImage(item.customer_image),
      vehicleNumber: item.vehicle_register_number,
      startTime: item.release_time,
      endTime: item.handover_time,
      status: item.order_status,
      date: item.order_date || new Date().toISOString().split('T')[0]
    }));
    console.log(todayOrders.value);
  } catch (error) {
    console.error('Error fetching orders:', error);
  }
};

// Lifecycle
onMounted(() => {
  fetchingData();
});
</script>

<style scoped>
.dashboard-page {
  min-height: 100vh;
  background: var(--color-background-tertiary);
}

.dashboard-content {
  padding: 2rem 0;
}

.dashboard-container {
  max-width: 1400px;
  margin: 0 auto;
  padding: 0 2rem;
}

/* Page Header */
.page-header {
  margin-bottom: 2.5rem;
}

.page-header h1 {
  margin: 0 0 0.5rem 0;
  font-size: 28px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.page-header p {
  margin: 0;
  font-size: 14px;
  color: var(--color-text-secondary);
}

/* Section */
.section {
  margin-bottom: 3rem;
}

.section-header {
  margin-bottom: 1.5rem;
}

.section-header h2 {
  margin: 0 0 0.5rem 0;
  font-size: 18px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.section-header p {
  margin: 0;
  font-size: 13px;
  color: var(--color-text-secondary);
}

/* Search Wrapper - Updated */
.search-wrapper {
  display: flex;
  gap: 1rem;
  margin-bottom: 2rem;
  flex-wrap: wrap;
}

.search-parameter-group {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.search-param-label {
  font-size: 14px;
  font-weight: 500;
  color: var(--color-text-primary);
  white-space: nowrap;
}

.search-parameter-select {
  padding: 0.75rem 1rem;
  background: var(--color-background-primary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-md);
  font-size: 14px;
  color: var(--color-text-primary);
  cursor: pointer;
  transition: all 0.2s ease;
}

.search-parameter-select:focus {
  outline: none;
  border-color: var(--color-border-secondary);
  background: var(--color-background-secondary);
}

.search-parameter-select option {
  background: var(--color-background-primary);
  color: var(--color-text-primary);
}

.search-input-group {
  flex: 1;
  min-width: 200px;
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem 1rem;
  background: var(--color-background-primary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-md);
  transition: all 0.2s ease;
}

.search-input-group:focus-within {
  border-color: var(--color-border-secondary);
  background: var(--color-background-secondary);
}

.search-input-group i {
  color: var(--color-text-secondary);
  font-size: 16px;
}

.search-input {
  flex: 1;
  border: none;
  background: transparent;
  padding: 0;
  font-size: 14px;
  color: var(--color-text-primary);
  outline: none;
}

.search-input::placeholder {
  color: var(--color-text-secondary);
}

.search-input::-webkit-calendar-picker-indicator {
  cursor: pointer;
  color: var(--color-text-secondary);
}

.search-button {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  padding: 0.75rem 1.5rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: var(--border-radius-md);
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s ease;
  white-space: nowrap;
}

.search-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
}

.search-button:active {
  transform: translateY(0);
}

.search-button i {
  font-size: 14px;
}

/* Search Results Section */
.search-results-section {
  margin-top: 2rem;
  padding-top: 2rem;
  border-top: 2px solid var(--color-border-tertiary);
}

.results-title {
  margin: 0 0 1.5rem 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.results-count {
  font-size: 14px;
  font-weight: 400;
  color: var(--color-text-secondary);
}

/* Cards Grid */
.cards-grid {
  display: grid;
  grid-template-columns: repeat(6, 1fr);
  gap: 1rem;
}

@media (max-width: 1600px) {
  .cards-grid {
    grid-template-columns: repeat(5, 1fr);
  }
}

@media (max-width: 1200px) {
  .cards-grid {
    grid-template-columns: repeat(4, 1fr);
  }
}

@media (max-width: 768px) {
  .cards-grid {
    grid-template-columns: repeat(3, 1fr);
  }
}

/* Order Card */
.order-card {
  background: var(--color-background-primary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-lg);
  overflow: hidden;
  transition: all 0.2s ease;
  display: flex;
  flex-direction: column;
}

.order-card:hover {
  border-color: var(--color-border-secondary);
  background: var(--color-background-secondary);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.06);
  transform: translateY(-2px);
}

/* Customer Photo */
.customer-photo {
  width: 100%;
  aspect-ratio: 4 / 3;
  overflow: hidden;
  background: var(--color-background-secondary);
  cursor: pointer;
  position: relative;
}

.photo-container {
  width: 100%;
  height: 100%;
  position: relative;
  display: flex;
  align-items: center;
  justify-content: center;
}

.card-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
  object-position: center;
  transition: transform 0.3s ease;
  backface-visibility: hidden;
  -webkit-backface-visibility: hidden;
}

.customer-photo:hover .card-image {
  transform: scale(1.05);
}

.photo-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0);
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background 0.3s ease;
}

.customer-photo:hover .photo-overlay {
  background: rgba(0, 0, 0, 0.3);
}

.photo-overlay i {
  color: white;
  font-size: 28px;
  opacity: 0;
  transition: opacity 0.3s ease;
  transform: scale(0.8);
}

.customer-photo:hover .photo-overlay i {
  opacity: 1;
  transform: scale(1);
}

/* Card Content */
.card-content {
  padding: 0.875rem;
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 0.625rem;
}

/* Status Wrapper */
.status-wrapper {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 0.5rem;
}

.status-wrapper > div {
  flex: 1;
}

.label {
  margin: 0 0 0.15rem 0;
  font-size: 10px;
  color: var(--color-text-secondary);
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.3px;
}

.customer-id {
  margin: 0;
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-primary);
}

/* Status Badge */
.status-badge {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  font-size: 10px;
  font-weight: 600;
  white-space: nowrap;
  text-transform: capitalize;
}

.status-badge.active {
  background: #d1fae5;
  color: #065f46;
}

.status-badge.pending {
  background: #fef3c7;
  color: #78350f;
}

.status-badge.completed {
  background: #fee2e2;
  color: #7f1d1d;
}

.status-badge.cancelled {
  background: #f3f4f6;
  color: #4b5563;
}

/* Info Section */
.info-section {
  padding-top: 0.625rem;
  border-top: 1px solid var(--color-border-tertiary);
}

.vehicle-number {
  margin: 0;
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-primary);
}

/* Time Info */
.time-info {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 0.5rem;
  padding-top: 0.625rem;
  border-top: 1px solid var(--color-border-tertiary);
}

.time-column p {
  margin: 0;
}

.time-value {
  font-size: 12px;
  font-weight: 600;
  color: var(--color-text-primary);
}

/* Empty State */
.empty-state {
  text-align: center;
  padding: 3rem 2rem;
  background: var(--color-background-primary);
  border: 1px dashed var(--color-border-tertiary);
  border-radius: var(--border-radius-lg);
}

.empty-state i {
  display: block;
  font-size: 48px;
  color: var(--color-text-secondary);
  margin-bottom: 1rem;
  opacity: 0.5;
}

.empty-state p {
  margin: 0;
  font-size: 15px;
  color: var(--color-text-secondary);
  font-weight: 500;
}

/* Image Modal Styles */
.image-modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 1rem;
  backdrop-filter: blur(4px);
  -webkit-backdrop-filter: blur(4px);
}

.image-modal-content {
  background: var(--color-background-primary);
  border-radius: var(--border-radius-lg);
  overflow: hidden;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  max-width: 90vw;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
  animation: slideUp 0.3s ease;
  position: relative;
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.modal-close-btn {
  position: absolute;
  top: 1rem;
  right: 1rem;
  background: rgba(0, 0, 0, 0.5);
  color: white;
  border: none;
  width: 44px;
  height: 44px;
  border-radius: 50%;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
  transition: all 0.2s ease;
  z-index: 10;
  backdrop-filter: blur(8px);
  -webkit-backdrop-filter: blur(8px);
}

.modal-close-btn:hover {
  background: rgba(0, 0, 0, 0.8);
  transform: scale(1.1);
}

.modal-close-btn:active {
  transform: scale(0.95);
}

.modal-image-wrapper {
  flex: 1;
  overflow: auto;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--color-background-secondary);
  min-height: 300px;
}

.modal-image {
  max-width: 100%;
  max-height: 100%;
  width: auto;
  height: auto;
  display: block;
  object-fit: contain;
}

.modal-info {
  padding: 1.5rem;
  background: var(--color-background-primary);
  border-top: 1px solid var(--color-border-tertiary);
}

.modal-info h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--color-text-primary);
}

/* Modal Transition */
.modal-fade-enter-active,
.modal-fade-leave-active {
  transition: opacity 0.3s ease;
}

.modal-fade-enter-from,
.modal-fade-leave-to {
  opacity: 0;
}

.modal-fade-enter-active .image-modal-content,
.modal-fade-leave-active .image-modal-content {
  transition: transform 0.3s ease;
}

.modal-fade-enter-from .image-modal-content {
  transform: translateY(20px);
}

.modal-fade-leave-to .image-modal-content {
  transform: translateY(20px);
}

/* View Button */
.view-button {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.6rem;
  padding: 0.8rem 1.25rem;
  margin-top: 0.5rem;
  background: linear-gradient(135deg, #728bdf 0%, #1e3a8a 100%);
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 600;
  letter-spacing: 0.3px;
  cursor: pointer;
  transition: all 0.2s ease;
  box-shadow: 0 2px 8px rgba(30, 64, 175, 0.2);
}

.view-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(30, 64, 175, 0.3);
}

.view-button:active {
  transform: translateY(0);
}

.view-button i {
  font-size: 14px;
  transition: transform 0.2s ease;
}

.view-button:hover i {
  transform: translateX(3px);
}

/* Responsive Design */
@media (max-width: 1024px) {
  .cards-grid {
    grid-template-columns: repeat(3, 1fr) !important;
  }
}

@media (max-width: 768px) {
  .dashboard-container {
    padding: 0 1rem;
  }

  .dashboard-content {
    padding: 1.5rem 0;
  }

  .page-header h1 {
    font-size: 24px;
  }

  .cards-grid {
    grid-template-columns: repeat(2, 1fr) !important;
    gap: 0.75rem;
  }

  .search-wrapper {
    flex-direction: column;
  }

  .search-parameter-group {
    width: 100%;
    flex-direction: column;
    align-items: flex-start;
  }

  .search-parameter-select {
    width: 100%;
  }

  .search-input-group {
    width: 100%;
  }

  .search-button {
    width: 100%;
  }

  .image-modal-content {
    max-width: 95vw;
    max-height: 80vh;
  }

  .modal-info {
    padding: 1rem;
  }

  .modal-info h3 {
    font-size: 14px;
  }
}

@media (max-width: 480px) {
  .page-header h1 {
    font-size: 20px;
  }

  .cards-grid {
    grid-template-columns: 1fr !important;
    gap: 0.75rem;
  }

  .status-wrapper {
    flex-direction: column;
  }

  .status-badge {
    align-self: flex-start;
  }

  .image-modal-overlay {
    padding: 0.5rem;
  }

  .image-modal-content {
    max-width: 100%;
    max-height: 85vh;
    border-radius: var(--border-radius-md);
  }

  .modal-close-btn {
    width: 40px;
    height: 40px;
    font-size: 18px;
  }

  .search-wrapper {
    gap: 0.5rem;
  }
}
</style>
