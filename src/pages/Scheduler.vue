<template>
  <div class="scheduler-page">
    <header-component
        :userName="`Logged in as ${loggedUser}`"
        :showBackButton="true"
        @logout="handleLogout"
    />

    <div class="scheduler-content">
      <div class="scheduler-container">
        <div class="page-header">
          <div>
            <h1>Booking Scheduler</h1>
            <p class="page-subtitle">See reservations at a glance and open any booking for details.</p>
          </div>

          <div class="header-controls">
            <div class="search-group">
              <label for="vehicleSelect">Vehicle</label>
              <select v-model="selectedVehicleId" id="vehicleSelect" class="vehicle-select">
                <option value="">All Vehicles</option>
                <option
                    v-for="vehicle in vehicles"
                    :key="vehicle.vehicleId"
                    :value="vehicle.vehicleId"
                >
                  {{ vehicle.registerNumber }} - {{ vehicle.manufacturer }} {{ vehicle.modelName }}
                </option>
              </select>
            </div>

            <div class="view-switcher" aria-label="Calendar view">
              <button
                  v-for="option in viewOptions"
                  :key="option.value"
                  type="button"
                  :class="['view-option', { active: viewMode === option.value }]"
                  @click="setViewMode(option.value)"
              >
                <i :class="option.icon"></i>
                {{ option.label }}
              </button>
            </div>
          </div>
        </div>

        <div class="calendar-toolbar">
          <button class="btn-nav" type="button" @click="previousPeriod" title="Previous">
            <i class="fa fa-chevron-left"></i>
          </button>

          <div class="period-display">
            <div class="period-title">{{ periodTitle }}</div>
            <button class="btn-today" type="button" @click="goToToday">Today</button>
          </div>

          <button class="btn-nav" type="button" @click="nextPeriod" title="Next">
            <i class="fa fa-chevron-right"></i>
          </button>
        </div>

        <div class="calendar-section">
          <!-- WEEK -->
          <div v-if="viewMode === 'week'" class="calendar-wrapper">
            <div class="week-grid">
              <div
                  v-for="day in daysOfWeek"
                  :key="`week-${day.key}`"
                  :class="['day-column', { 'is-today': isToday(day.date) }]"
              >
                <button class="day-header" type="button" @click="openDay(day.date)">
                  <span class="day-name">{{ day.name }}</span>
                  <span class="day-date">{{ formatDayDate(day.date) }}</span>
                  <span class="event-count">{{ getEventsForDate(day.date).length }}</span>
                </button>

                <div class="day-events">
                  <button
                      v-for="event in getEventsForDate(day.date)"
                      :key="event.id"
                      :class="['event-card', `status-${convertSnakeCase(event.reservationStatus)}`]"
                      type="button"
                      @click="viewEvent(event)"
                  >
                    <span class="event-title">{{ event.customerName || 'Unnamed customer' }}</span>
                    <span class="event-meta">{{ event.vehicleNumber || 'Vehicle not assigned' }}</span>
                    <span class="event-meta">{{ event.startTime.split("T")[1] || 'No time'}}</span>
                    <span class="event-meta">{{ event.contactNo || 'No contact number' }}</span>
                  </button>

                  <div v-if="!getEventsForDate(day.date).length" class="empty-day">
                    No bookings
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- MONTH -->
          <div v-else-if="viewMode === 'month'" class="calendar-wrapper">
            <div class="month-grid">
              <div v-for="dayName in weekDayNames" :key="dayName" class="month-weekday">
                {{ dayName }}
              </div>

              <button
                  v-for="day in monthDays"
                  :key="`month-${day.key}`"
                  :class="[
                  'month-day',
                  {
                    'is-today': day.isToday,
                    'is-other-month': !day.isCurrentMonth,
                    'has-events': day.events.length
                  }
                ]"
                  type="button"
                  @click="openDay(day.date)"
              >
                <div class="month-day-top">
                  <span class="month-day-number">{{ day.date.getDate() }}</span>
                  <span v-if="day.events.length" class="month-event-count">
                    {{ day.events.length }}
                  </span>
                </div>

                <div class="month-event-list">
                  <span
                      v-for="event in day.events.slice(0, 3)"
                      :key="event.id"
                      :class="['month-event-dot', `status-${convertSnakeCase(event.reservationStatus)}`]"
                  >
                    {{ event.customerName || 'Booking' }}
                  </span>
                  <span v-if="day.events.length > 3" class="more-events">
                    +{{ day.events.length - 3 }} more
                  </span>
                </div>
              </button>
            </div>
          </div>

          <!-- DAY -->
          <div v-else class="calendar-wrapper day-view-wrapper">
            <div class="day-view">
              <div class="day-view-header">
                <div>
                  <div class="eyebrow">Selected day</div>
                  <h2>{{ formatLongDate(currentDate) }}</h2>
                </div>
                <span class="day-total">{{ selectedDayEvents.length }} bookings</span>
              </div>

              <div v-if="selectedDayEvents.length" class="day-event-list">
                <button
                    v-for="event in selectedDayEvents"
                    :key="event.id"
                    :class="['event-row', `status-${convertSnakeCase(event.reservationStatus)}`]"
                    type="button"
                    @click="viewEvent(event)"
                >
                  <span class="event-row-main">
                    <strong>{{ event.customerName || 'Unnamed customer' }}</strong>
                    <span>{{ event.vehicleNumber || 'Vehicle not assigned' }}</span>
                    <span>{{event.startTime.split("T")[1] || 'Not Start time'}}</span>
                  </span>
                  <span class="event-row-details">
                    <span>{{ event.contactNo || 'No contact number' }}</span>
                    <span :class="['status-badge', `status-${convertSnakeCase(event.reservationStatus)}`]">
                      {{ formatStatus(event.reservationStatus) }}
                    </span>
                  </span>
                  <i class="fa fa-chevron-right"></i>
                </button>
              </div>

              <div v-else class="day-empty-state">
                <i class="fa fa-calendar-o"></i>
                <h3>No bookings for this day</h3>
                <p>There are no reservations matching the current vehicle filter.</p>
              </div>
            </div>
          </div>
        </div>

        <div class="footer-section">
          <div class="legend">
            <h3>Status</h3>
            <div class="legend-items">
              <div class="legend-item">
                <span class="legend-dot status-active"></span>
                <span>Active</span>
              </div>
              <div class="legend-item">
                <span class="legend-dot status-completed"></span>
                <span>Completed</span>
              </div>
              <div class="legend-item">
                <span class="legend-dot status-reserved"></span>
                <span>Reserved</span>
              </div>
            </div>
          </div>

          <div class="stats">
            <div class="stat-item">
              <label>{{ statsPeriodLabel }} bookings</label>
              <span>{{ periodEvents.length }}</span>
            </div>
            <div class="stat-item">
              <label>Active</label>
              <span class="active">{{ periodActiveCount }}</span>
            </div>
            <div class="stat-item">
              <label>Completed</label>
              <span class="completed">{{ periodCompletedCount }}</span>
            </div>
            <div class="stat-item">
              <label>Vehicle</label>
              <span>{{ selectedVehicleName }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>

  </div>
</template>

<script>
import HeaderComponent from '../component/Header.vue';
import { dbService } from '../services/db.ts';
import { useRouter } from 'vue-router'
import { useAuthStore } from "../stores/auth.ts";
import {convertSnakeCase, orderStatus, paymentStatus, paymentTypes} from "../utils/constants.ts";

export default {
  name: 'VehicleScheduler',
  components: {
    HeaderComponent
  },

  data() {
    return {
      loggedUser: null,
      selectedVehicleId: '',
      currentDate: new Date(),
      viewMode: 'week',
      vehicles: [],
      reservations: [],
      showEventModal: false,
      selectedEvent: null,
      editingReservation: null,
      advancedPaymentAmount: 0,
      router: useRouter(),
      viewOptions: [
        { value: 'day', label: 'Day', icon: 'fa fa-calendar-day' },
        { value: 'week', label: 'Week', icon: 'fa fa-calendar-week' },
        { value: 'month', label: 'Month', icon: 'fa fa-calendar' }
      ],

      weekDayNames: ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat']
    };
  },
  created() {
    const authStore = useAuthStore()
    this.loggedUser = authStore.username
    this.roleName = authStore.role
  },
  computed: {
    paymentTypes() {
      return paymentTypes
    },
    orderStatus() {
      return orderStatus
    },
    daysOfWeek() {
      const days = [];
      const weekStart = this.getWeekStart(this.currentDate);

      for (let i = 0; i < 7; i++) {
        const date = new Date(weekStart);
        date.setDate(date.getDate() + i);

        days.push({
          date,
          key: this.dateKey(date),
          name: this.weekDayNames[date.getDay()]
        });
      }

      return days;
    },

    monthDays() {
      const firstOfMonth = new Date(
          this.currentDate.getFullYear(),
          this.currentDate.getMonth(),
          1
      );
      const start = this.getWeekStart(firstOfMonth);
      const days = [];

      for (let i = 0; i < 42; i++) {
        const date = new Date(start);
        date.setDate(start.getDate() + i);

        days.push({
          date,
          key: this.dateKey(date),
          isCurrentMonth: date.getMonth() === this.currentDate.getMonth(),
          isToday: this.isToday(date),
          events: this.getEventsForDate(date)
        });
      }

      return days;
    },

    filteredReservations() {
      if (!this.selectedVehicleId) {
        return this.reservations;
      }
      console.log(this.reservations);
      return this.reservations.filter(res => res.vehicleId == this.selectedVehicleId);
    },

    weeklyEvents() {
      const weekStart = this.getWeekStart(this.currentDate);
      const weekEnd = new Date(weekStart);
      weekEnd.setDate(weekEnd.getDate() + 7);

      return this.filteredReservations.filter(res => {
        const eventDate = new Date(res.startTime);
        return eventDate >= weekStart && eventDate < weekEnd;
      });
    },

    monthlyEvents() {
      const year = this.currentDate.getFullYear();
      const month = this.currentDate.getMonth();

      return this.filteredReservations.filter(res => {
        const eventDate = new Date(res.startTime);
        return eventDate.getFullYear() === year && eventDate.getMonth() === month;
      });
    },

    selectedDayEvents() {
      return this.getEventsForDate(this.currentDate);
    },

    periodEvents() {
      if (this.viewMode === 'day') return this.selectedDayEvents;
      if (this.viewMode === 'month') return this.monthlyEvents;
      return this.weeklyEvents;
    },

    periodActiveCount() {
      return this.periodEvents.filter(e => e.reservationStatus === orderStatus.active).length;
    },

    periodCompletedCount() {
      return this.periodEvents.filter(e => e.reservationStatus === orderStatus.completed).length;
    },

    statsPeriodLabel() {
      if (this.viewMode === 'day') return 'Day';
      if (this.viewMode === 'month') return 'Month';
      return 'Week';
    },

    selectedVehicleName() {
      if (!this.selectedVehicleId) return 'All Vehicles';

      const vehicle = this.vehicles.find(v => v.vehicleId == this.selectedVehicleId);
      return vehicle ? vehicle.registerNumber : 'Unknown';
    },

    periodTitle() {
      if (this.viewMode === 'day') {
        return this.formatLongDate(this.currentDate);
      }

      if (this.viewMode === 'month') {
        return this.currentDate.toLocaleString('en-US', {
          month: 'long',
          year: 'numeric'
        });
      }

      return `${this.formatDateRange(this.getWeekStart(this.currentDate))} - ${this.formatDateRange(this.getWeekEnd(this.currentDate))}`;
    }
  },

  methods: {
    convertSnakeCase,

    dateKey(date) {
      const d = new Date(date);
      const year = d.getFullYear();
      const month = String(d.getMonth() + 1).padStart(2, '0');
      const day = String(d.getDate()).padStart(2, '0');
      return `${year}-${month}-${day}`;
    },

    getEventsForDate(date) {
      const key = this.dateKey(date);

      return this.filteredReservations
          .filter(event => this.dateKey(event.startTime) === key)
          .sort((a, b) => new Date(a.startTime) - new Date(b.startTime));
    },

    formatPrice(price) {
      return new Intl.NumberFormat('en-US', {
        style: 'currency',
        currency: 'LKR',
        minimumFractionDigits: 2,
        maximumFractionDigits: 2
      }).format(price || 0);
    },
    calculateTotalDistance() {
      if (this.editingReservation && this.editingReservation.endMileage && this.editingReservation.startMileage) {
        this.totalDistance = this.editingReservation.endMileage - this.editingReservation.startMileage;
        return this.totalDistance;
      }
      return 0;
    },
    calculateTotalAmount() {
      if (this.editingReservation && this.editingReservation.endMileage && this.editingReservation.startMileage && this.totalDistance > 200) {
        const orderedVehicles = this.vehicles.find(v => {
          return v.vehicle_id === this.editingReservation.vehicleId
        });
        this.totalAmount = (this.totalDistance - 200) * orderedVehicles.unit_price + orderedVehicles.base_price;
        return this.totalAmount;
      }
      return this.editingReservation.totalAmount;
    },

    formatDateRange(date) {
      return date.toLocaleString('en-US', {
        year: 'numeric',
        month: 'short',
        day: 'numeric'
      });
    },

    formatDayDate(date) {
      return date.toLocaleString('en-US', {
        month: 'short',
        day: 'numeric'
      });
    },

    formatLongDate(date) {
      return date.toLocaleString('en-US', {
        weekday: 'long',
        year: 'numeric',
        month: 'long',
        day: 'numeric'
      });
    },

    formatStatus(status) {
      if (!status) return 'Unknown';
      return status.charAt(0).toUpperCase() + status.slice(1);
    },

    formatPaymentStatus(status) {
      if (status === 'paid') return 'Paid';
      if (status === 'pending') return 'Pending';
      if (status === 'cancelled') return 'Cancelled';
      return status;
    },

    calculateDuration(startTime, endTime) {
      if (!this.editingReservation || !this.editingReservation.startTime || !this.editingReservation.endTime) {
        return '-';
      }
      const start = new Date(this.editingReservation.startTime);
      const end = new Date(this.editingReservation.endTime);
      const diffMs = end - start;
      const diffHours = Math.floor(diffMs / (1000 * 60 * 60));
      const diffMinutes = Math.floor((diffMs % (1000 * 60 * 60)) / (1000 * 60));
      return `${diffHours}h ${diffMinutes}m`;
    },

    getWeekStart(date) {
      const d = new Date(date);
      const day = d.getDay();
      const diff = d.getDate() - day;
      d.setDate(diff);
      d.setHours(0, 0, 0, 0);
      return d;
    },

    getWeekEnd(date) {
      const end = this.getWeekStart(date);
      end.setDate(end.getDate() + 6);
      return end;
    },

    isToday(date) {
      const today = new Date();
      return this.dateKey(date) === this.dateKey(today);
    },

    setViewMode(mode) {
      this.viewMode = mode;
    },

    openDay(date) {
      this.currentDate = new Date(date);
      this.viewMode = 'day';
    },

    previousPeriod() {
      const date = new Date(this.currentDate);

      if (this.viewMode === 'day') {
        date.setDate(date.getDate() - 1);
      } else if (this.viewMode === 'month') {
        date.setMonth(date.getMonth() - 1);
      } else {
        date.setDate(date.getDate() - 7);
      }

      this.currentDate = date;
    },

    nextPeriod() {
      const date = new Date(this.currentDate);

      if (this.viewMode === 'day') {
        date.setDate(date.getDate() + 1);
      } else if (this.viewMode === 'month') {
        date.setMonth(date.getMonth() + 1);
      } else {
        date.setDate(date.getDate() + 7);
      }

      this.currentDate = date;
    },

    goToToday() {
      this.currentDate = new Date();
    },

    viewEvent(event) {
      this.editingReservation = JSON.parse(JSON.stringify(event));
      this.router.push({
        name: 'view-order-detail',
        params: { id: this.editingReservation.reservationId },
      })
    },

    closeEventModal() {
      this.showEventModal = false;
      this.editingReservation = null;
    },

    handleLogout() {
      this.$emit('logout');
    },

    /*
     * API calls intentionally unchanged.
     */
    async getVehicles() {
      try {
        const result = await dbService.getVehicles();
        this.vehicles = result.map((item) => ({
          id: item.id,
          vehicleId: item.vehicle_id,
          registerNumber: item.register_number,
          manufacturer: item.manufacturer,
          modelName: item.model_name
        }));
      } catch (error) {
        console.log('Error fetching vehicles:', error);
      }
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
        this.editingReservation[fieldName] = Array.from(byteArray);

        // Create preview
        const blob = new Blob([byteArray], {
          type: file.type,
        });

        this.editingReservation[previewFieldName] = URL.createObjectURL(blob);
      };
      reader.readAsArrayBuffer(file);

      // Reset input
      event.target.value = '';
    },

    /**
     * Clear uploaded image
     */
    clearImage(fieldName) {
      this.editingReservation[fieldName] = null;
      this.reservations[fieldName] = null;
    },
    getCustomerImage(imageArray){
      if (imageArray && imageArray.length > 0) {
        const bytes = new Uint8Array(imageArray)
        const blob = new Blob(
            [bytes],
            { type: 'image/jpg' }
        )
        return URL.createObjectURL(blob);
      }
    },
    async getReservations() {
      try {
        const result = await dbService.getAllOrders();
        this.reservations = result.map((item) => ({
          id: item.id,
          orderNumber: item.orderNumber,
          reservationId: item.orderNumber,
          customerId: item.customerId,
          customerName: item.customerName,
          vehicleId: item.vehicleId,
          vehicleNumber: this.vehicles.find(v => v.vehicleId === item.vehicleId).registerNumber,
          totalAmount: item.totalAmount,
          paidAmount: item.paidAmount,
          customerImage: item.customerImage,
          customerImagePreview: this.getCustomerImage(item.customerImage),
          guaranteeProperty: item.guaranteeProperty,
          guaranteeType: item.guaranteeType,
          reservationStatus: item.orderStatus,
          paymentStatus: item.paymentStatus,
          startTime: item.releaseTime,
          endTime: item.handoverTime,
          startMileage: item.startingMileage,
          endMileage: item.endMileage,
          notes: item.notes || '',
          contactNo: item.contactNo,
        }));
      } catch (error) {
        console.log('Error fetching reservations:', error);
      }
    }
  },

  async mounted() {
    await this.getVehicles();
    await this.getReservations();
  }
};
</script>

<style scoped>
.scheduler-page {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
  background: var(--color-background-tertiary);
  color: var(--color-text-primary);
}

.scheduler-content {
  flex: 1;
  overflow: hidden;
}

.scheduler-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  max-width: 1600px;
  margin: 0 auto;
  padding: 1.5rem;
  box-sizing: border-box;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-end;
  gap: 2rem;
  margin-bottom: 1.25rem;
}

.eyebrow {
  margin-bottom: 0.35rem;
  font-size: 0.7rem;
  font-weight: 700;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: var(--color-text-secondary);
}

.page-header h1 {
  margin: 0;
  font-size: 28px;
  line-height: 1.15;
  font-weight: 700;
  color: var(--color-text-primary);
}

.page-subtitle {
  margin: 0.5rem 0 0;
  color: var(--color-text-secondary);
  font-size: 13px;
}

.header-controls {
  display: flex;
  align-items: center;
  gap: 1rem;
  flex-wrap: wrap;
}

.search-group {
  display: flex;
  align-items: center;
  gap: 0.65rem;
}

.search-group label {
  font-size: 12px;
  font-weight: 700;
  color: var(--color-text-secondary);
}

.vehicle-select {
  min-width: 230px;
  padding: 0.65rem 2.2rem 0.65rem 0.8rem;
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-md);
  background: var(--color-background-primary);
  color: var(--color-text-primary);
  font-size: 13px;
  cursor: pointer;
}

.vehicle-select:focus {
  outline: none;
  border-color: var(--color-info);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.view-switcher {
  display: flex;
  padding: 3px;
  gap: 2px;
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-md);
  background: var(--color-background-secondary);
}

.view-option {
  border: 0;
  border-radius: 5px;
  padding: 0.55rem 0.8rem;
  background: transparent;
  color: var(--color-text-secondary);
  font-size: 12px;
  font-weight: 700;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.view-option i {
  margin-right: 0.35rem;
}

.view-option:hover {
  color: var(--color-text-primary);
  background: var(--color-background-primary);
}

.view-option.active {
  background: var(--color-background-primary);
  color: var(--color-info);
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.08);
}

.calendar-toolbar {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 1rem;
  min-height: 54px;
  margin-bottom: 1rem;
}

.btn-nav {
  width: 38px;
  height: 38px;
  border: 1px solid var(--color-border-tertiary);
  border-radius: 8px;
  background: var(--color-background-primary);
  color: var(--color-text-primary);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.btn-nav:hover {
  border-color: var(--color-info);
  color: var(--color-info);
}

.period-display {
  display: flex;
  align-items: center;
  gap: 0.8rem;
  min-width: 310px;
  justify-content: center;
}

.period-title {
  font-size: 15px;
  font-weight: 700;
  color: var(--color-text-primary);
  text-align: center;
}

.btn-today {
  padding: 0.45rem 0.75rem;
  border: 1px solid var(--color-border-tertiary);
  border-radius: 6px;
  background: var(--color-background-primary);
  color: var(--color-info);
  font-size: 11px;
  font-weight: 700;
  cursor: pointer;
}

.btn-today:hover {
  background: var(--color-background-secondary);
}

.calendar-section {
  flex: 1;
  min-height: 0;
  overflow: hidden;
  background: var(--color-background-primary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-lg);
  margin-bottom: 1rem;
}

.calendar-wrapper {
  width: 100%;
  height: 100%;
  overflow: auto;
}

/* WEEK VIEW — intentionally no time slots/time axis. */
.week-grid {
  display: grid;
  grid-template-columns: repeat(7, minmax(160px, 1fr));
  min-width: 1050px;
  min-height: 100%;
}

.day-column {
  min-height: 100%;
  border-right: 1px solid var(--color-border-tertiary);
}

.day-column:last-child {
  border-right: 0;
}

.day-header {
  position: sticky;
  top: 0;
  z-index: 4;
  width: 100%;
  display: flex;
  align-items: center;
  gap: 0.45rem;
  padding: 0.9rem 0.75rem;
  border: 0;
  border-bottom: 1px solid var(--color-border-tertiary);
  background: var(--color-background-secondary);
  color: var(--color-text-primary);
  text-align: left;
  cursor: pointer;
}

.day-header:hover {
  background: var(--color-background-primary);
}

.day-column.is-today .day-header {
  box-shadow: inset 0 3px 0 var(--color-info);
}

.day-name {
  font-size: 12px;
  font-weight: 700;
}

.day-date {
  font-size: 11px;
  color: var(--color-text-secondary);
}

.event-count {
  margin-left: auto;
  min-width: 22px;
  height: 22px;
  padding: 0 5px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: 11px;
  background: var(--color-background-primary);
  color: var(--color-text-secondary);
  font-size: 10px;
  font-weight: 700;
}

.day-events {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  padding: 0.65rem;
}

.event-card {
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 0.22rem;
  padding: 0.65rem;
  border: 0;
  border-left: 3px solid;
  border-radius: 6px;
  text-align: left;
  cursor: pointer;
  transition: transform var(--transition-fast), box-shadow var(--transition-fast);
}

.event-card:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.09);
}

.event-card.status-active {
  background: #dbeafe;
  color: #1e40af;
  border-left-color: #1e40af;
}

.event-card.status-completed {
  background: #dcfce7;
  color: #15803d;
  border-left-color: #16a34a;
}

.event-card.status-reserved {
  background: #fee2e2;
  color: #b91c1c;
  border-left-color: #dc2626;
}

.event-title {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 12px;
  font-weight: 700;
}

.event-meta {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 10px;
  opacity: 0.8;
}

.empty-day {
  padding: 1.5rem 0.5rem;
  color: var(--color-text-secondary);
  font-size: 11px;
  text-align: center;
}

/* MONTH VIEW */
.month-grid {
  display: grid;
  grid-template-columns: repeat(7, minmax(120px, 1fr));
  grid-template-rows: auto repeat(6, minmax(105px, 1fr));
  min-width: 840px;
  min-height: 100%;
}

.month-weekday {
  position: sticky;
  top: 0;
  z-index: 4;
  padding: 0.7rem;
  border-right: 1px solid var(--color-border-tertiary);
  border-bottom: 1px solid var(--color-border-tertiary);
  background: var(--color-background-secondary);
  color: var(--color-text-secondary);
  font-size: 10px;
  font-weight: 800;
  text-transform: uppercase;
  letter-spacing: 0.06em;
}

.month-day {
  position: relative;
  min-width: 0;
  padding: 0.6rem;
  border: 0;
  border-right: 1px solid var(--color-border-tertiary);
  border-bottom: 1px solid var(--color-border-tertiary);
  background: var(--color-background-primary);
  color: var(--color-text-primary);
  text-align: left;
  cursor: pointer;
}

.month-day:hover {
  background: var(--color-background-secondary);
}

.month-day.is-other-month {
  background: rgba(0, 0, 0, 0.015);
  color: var(--color-text-secondary);
}

.month-day.is-today {
  box-shadow: inset 0 3px 0 var(--color-info);
}

.month-day-top {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 0.45rem;
}

.month-day-number {
  font-size: 12px;
  font-weight: 700;
}

.month-event-count {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 24px;
  height: 22px;
  padding: 0 5px;
  border-radius: 11px;
  background: var(--color-background-secondary);
  color: var(--color-info);
  font-size: 10px;
  font-weight: 800;
}

.month-event-list {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.month-event-dot {
  display: block;
  overflow: hidden;
  padding: 0.22rem 0.35rem;
  border-radius: 4px;
  text-overflow: ellipsis;
  white-space: nowrap;
  font-size: 9px;
  font-weight: 600;
}

.month-event-dot.status-active {
  background: #dbeafe;
  color: #1e40af;
}

.month-event-dot.status-completed {
  background: #dcfce7;
  color: #15803d;
}

.month-event-dot.status-reserved {
  background: #fee2e2;
  color: #b91c1c;
}

.more-events {
  color: var(--color-text-secondary);
  font-size: 9px;
  font-weight: 700;
}

/* DAY VIEW */
.day-view {
  max-width: 1000px;
  margin: 0 auto;
  padding: 1.25rem;
}

.day-view-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  padding-bottom: 1rem;
  border-bottom: 1px solid var(--color-border-tertiary);
}

.day-view-header h2 {
  margin: 0;
  font-size: 20px;
}

.day-total {
  padding: 0.4rem 0.7rem;
  border-radius: 20px;
  background: var(--color-background-secondary);
  color: var(--color-text-secondary);
  font-size: 11px;
  font-weight: 700;
}

.day-event-list {
  display: flex;
  flex-direction: column;
  gap: 0.55rem;
  padding-top: 1rem;
}

.event-row {
  width: 100%;
  display: grid;
  grid-template-columns: 1fr auto auto;
  align-items: center;
  gap: 1rem;
  padding: 0.85rem 1rem;
  border: 0;
  border-left: 4px solid;
  border-radius: 7px;
  background: var(--color-background-secondary);
  color: var(--color-text-primary);
  text-align: left;
  cursor: pointer;
}

.event-row:hover {
  background: var(--color-background-primary);
  box-shadow: 0 3px 12px rgba(0, 0, 0, 0.07);
}

.event-row.status-active {
  border-left-color: #1e40af;
}

.event-row.status-completed {
  border-left-color: #16a34a;
}

.event-row.status-reserved {
  border-left-color: #dc2626;
}

.event-row-main,
.event-row-details {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.event-row-main strong {
  font-size: 13px;
}

.event-row-main span,
.event-row-details span {
  color: var(--color-text-secondary);
  font-size: 11px;
}

.day-empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  min-height: 260px;
  color: var(--color-text-secondary);
  text-align: center;
}

.day-empty-state i {
  margin-bottom: 0.8rem;
  font-size: 30px;
}

.day-empty-state h3 {
  margin: 0;
  color: var(--color-text-primary);
  font-size: 15px;
}

.day-empty-state p {
  margin: 0.4rem 0 0;
  font-size: 12px;
}

/* FOOTER */
.footer-section {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 2rem;
  padding: 0.9rem 1rem;
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-md);
  background: var(--color-background-primary);
}

.legend h3 {
  margin: 0 0 0.5rem;
  font-size: 10px;
  font-weight: 800;
  text-transform: uppercase;
  letter-spacing: 0.06em;
  color: var(--color-text-secondary);
}

.legend-items,
.stats {
  display: flex;
  align-items: center;
  gap: 1rem;
  flex-wrap: wrap;
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 0.35rem;
  font-size: 11px;
}

.legend-dot {
  width: 9px;
  height: 9px;
  border-radius: 3px;
  border: 1px solid;
}

.legend-dot.status-active {
  background: #dbeafe;
  border-color: #1e40af;
}

.legend-dot.status-completed {
  background: #dcfce7;
  border-color: #16a34a;
}

.legend-dot.status-reserved {
  background: #fee2e2;
  border-color: #dc2626;
}

.stat-item {
  display: flex;
  flex-direction: column;
  gap: 0.15rem;
  min-width: 80px;
}

.stat-item label {
  color: var(--color-text-secondary);
  font-size: 9px;
  font-weight: 700;
  text-transform: uppercase;
}

.stat-item span {
  color: var(--color-text-primary);
  font-size: 15px;
  font-weight: 700;
}

.stat-item span.active {
  color: #1e40af;
}

.stat-item span.completed {
  color: #16a34a;
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
  max-width: 700px;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
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

.modal-header h2 {
  margin: 0;
  font-size: 20px;
  color: white;
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
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

/* === INFO SECTION === */
.info-section {
  padding-bottom: 1.5rem;
  border-bottom: 1px solid var(--color-border-tertiary);
}

.section-title {
  margin: 0 0 1rem 0;
  font-size: 15px;
  font-weight: 600;
  color: var(--color-text-primary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.info-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1.5rem;
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.info-item label {
  font-size: 12px;
  color: var(--color-text-secondary);
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.3px;
}

.info-item p {
  margin: 0;
  font-size: 14px;
  color: var(--color-text-primary);
  font-weight: 500;
}

/* === EDIT SECTION === */
.edit-section {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
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
  color: var(--color-text-primary);
  text-transform: capitalize;
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

.input-field.textarea {
  resize: vertical;
  font-family: inherit;
}

.input-field:read-only {
  background: var(--color-background-secondary);
  opacity: 0.7;
  cursor: not-allowed;
}

/* === SUMMARY SECTION === */
.summary-section {
  padding-top: 1.5rem;
  border-top: 1px solid var(--color-border-tertiary);
}

.summary-card {
  background: var(--color-background-secondary);
  border-radius: 8px;
  padding: 1rem;
}

.summary-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.6rem 0;
  font-size: 13px;
}

.summary-row label {
  color: var(--color-text-secondary);
  font-weight: 500;
}

.summary-row span {
  color: var(--color-text-primary);
  font-weight: 500;
}

.summary-row.final {
  padding-top: 0.8rem;
  border-top: 1px solid var(--color-border-tertiary);
  margin-top: 0.8rem;
  font-size: 14px;
}

.summary-row.final label {
  color: var(--color-text-primary);
  font-weight: 600;
}

.summary-row.final .amount {
  color: var(--color-success);
  font-weight: 700;
  font-family: monospace;
  font-size: 16px;
}

/* === MODAL FOOTER === */
.modal-footer {
  display: flex;
  gap: 1rem;
  padding: 1.5rem;
  border-top: 1px solid var(--color-border-tertiary);
  background: var(--color-background-secondary);
  border-radius: 0 0 var(--border-radius-lg) var(--border-radius-lg);
  justify-content: flex-end;
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
  background: var(--color-background-primary);
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
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.btn-primary:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
}

.btn-danger {
  padding: 0.75rem 1.5rem;
  background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
  color: white;
  border: none;
  border-radius: var(--border-radius-md);
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all var(--transition-fast);
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.btn-danger:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(239, 68, 68, 0.3);
}

/* Shared modal styles */
.status-badge,
.payment-badge {
  display: inline-block;
  width: fit-content;
  padding: 0.35rem 0.65rem;
  border-radius: 20px;
  font-size: 10px;
  font-weight: 700;
  text-transform: capitalize;
}

.status-badge.status-active {
  background: #dbeafe;
  color: #1e40af;
}

.status-badge.status-completed {
  background: #dcfce7;
  color: #16a34a;
}

.status-badge.status-reserved {
  background: #fee2e2;
  color: #dc2626;
}

.modal-overlay {
  position: fixed;
  inset: 0;
  z-index: 1000;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 1rem;
  background: rgba(0, 0, 0, 0.5);
}

.modal-content {
  width: min(650px, 95vw);
  max-height: 90vh;
  overflow-y: auto;
  background: var(--color-background-primary);
  border-radius: var(--border-radius-lg);
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
}

.modal-header {
  position: sticky;
  top: 0;
  z-index: 2;
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1.25rem 1.5rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

.modal-header h2 {
  margin: 0;
  font-size: 18px;
}

.btn-close {
  border: 0;
  background: transparent;
  color: white;
  font-size: 19px;
  cursor: pointer;
}

.modal-body {
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
  padding: 1.5rem;
}

.event-info-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1.5rem;
}

.info-column {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.info-group {
  display: flex;
  flex-direction: column;
  gap: 0.35rem;
}

.info-group label,
.mileage-item label,
.billing-item label {
  color: var(--color-text-secondary);
  font-size: 10px;
  font-weight: 700;
  text-transform: uppercase;
}

.info-group p,
.mileage-item p,
.billing-item p {
  margin: 0;
  color: var(--color-text-primary);
  font-size: 13px;
  font-weight: 500;
}

.mileage-section,
.billing-section,
.notes-section {
  padding: 1rem;
  border-radius: 8px;
  background: var(--color-background-secondary);
}

.mileage-section h3,
.billing-section h3,
.notes-section h3 {
  margin: 0 0 0.8rem;
  font-size: 12px;
  text-transform: uppercase;
  letter-spacing: 0.04em;
}

.mileage-grid,
.billing-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 1rem;
}

.billing-grid {
  grid-template-columns: 1fr 1fr;
}

.notes-section p {
  margin: 0;
  color: var(--color-text-primary);
  font-size: 12px;
  line-height: 1.6;
}

.amount {
  color: var(--color-success) !important;
  font-size: 16px !important;
  font-weight: 700 !important;
}

.payment-badge.payment-paid {
  background: #dcfce7;
  color: #16a34a;
}

.payment-badge.payment-pending {
  background: #fef3c7;
  color: #d97706;
}

.payment-badge.payment-cancelled {
  background: #fee2e2;
  color: #dc2626;
}

.modal-footer {
  position: sticky;
  bottom: 0;
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
  padding: 1rem 1.5rem;
  border-top: 1px solid var(--color-border-tertiary);
  background: var(--color-background-secondary);
}

.btn-secondary,
.btn-primary {
  padding: 0.65rem 1rem;
  border-radius: var(--border-radius-md);
  font-size: 12px;
  font-weight: 700;
  cursor: pointer;
}

.btn-secondary {
  border: 1px solid var(--color-border-tertiary);
  background: transparent;
  color: var(--color-text-primary);
}

.btn-primary {
  display: flex;
  align-items: center;
  gap: 0.4rem;
  border: 0;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

@media (max-width: 1000px) {
  .page-header {
    align-items: stretch;
    flex-direction: column;
    gap: 1rem;
  }

  .header-controls {
    justify-content: space-between;
  }

  .footer-section {
    align-items: flex-start;
    flex-direction: column;
  }
}

@media (max-width: 700px) {
  .scheduler-container {
    padding: 1rem;
  }

  .header-controls,
  .search-group {
    width: 100%;
  }

  .search-group {
    justify-content: space-between;
  }

  .vehicle-select {
    flex: 1;
    min-width: 0;
  }

  .view-switcher {
    width: 100%;
  }

  .view-option {
    flex: 1;
  }

  .period-display {
    min-width: 0;
    flex: 1;
  }

  .period-title {
    font-size: 13px;
  }

  .event-row {
    grid-template-columns: 1fr auto;
  }

  .event-row-details {
    display: none;
  }

  .event-info-grid,
  .mileage-grid,
  .billing-grid {
    grid-template-columns: 1fr;
  }

  .modal-footer {
    flex-direction: column;
  }

  .modal-footer button {
    width: 100%;
  }
}
</style>
