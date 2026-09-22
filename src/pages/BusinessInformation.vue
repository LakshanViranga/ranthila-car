<template>
  <div class="business-information-page">
    <!-- Header -->
    <header-component
        :showBackButton="true"
        @logout="handleLogout"
    />

    <!-- Content -->
    <div class="business-information-content">
      <div class="business-information-container">

        <!-- Page Header -->
        <div class="page-header">
          <div>
            <h1>Business Information</h1>
            <p>Manage your rent-a-car business details and operating hours</p>
          </div>
        </div>

        <form @submit.prevent="saveBusinessInformation">

          <!-- Business Profile -->
          <section class="information-section">

            <div class="section-header">
              <div class="section-icon business">
                <i class="fa fa-building"></i>
              </div>

              <div>
                <h2>Business Profile</h2>
                <p>Basic information about your rental business</p>
              </div>
            </div>

            <div class="form-content">

              <div class="form-group">
                <label for="businessName">
                  Business Name
                  <span class="required">*</span>
                </label>

                <input
                    id="businessName"
                    v-model="form.businessName"
                    type="text"
                    placeholder="Enter business name"
                    required
                />
              </div>

              <div class="form-row">

                <div class="form-group">
                  <label for="location">
                    Location / City
                    <span class="required">*</span>
                  </label>

                  <input
                      id="location"
                      v-model="form.location"
                      type="text"
                      placeholder="Enter location"
                      required
                  />
                </div>

                <div class="form-group">
                  <label for="businessType">
                    Business Type
                  </label>

                  <input
                      id="businessType"
                      v-model="form.businessType"
                      type="text"
                      placeholder="Vehicle Rental"
                  />
                </div>

              </div>

            </div>
          </section>


          <!-- Contact Information -->
          <section class="information-section">

            <div class="section-header">
              <div class="section-icon contact">
                <i class="fa fa-phone"></i>
              </div>

              <div>
                <h2>Contact Information</h2>
                <p>Contact details displayed on receipts and rental documents</p>
              </div>
            </div>

            <div class="form-content">

              <div class="form-row">

                <div class="form-group">
                  <label for="phone">
                    Phone Number
                  </label>

                  <input
                      id="phone"
                      v-model="form.phone"
                      type="tel"
                      placeholder="+94 XX XXX XXXX"
                  />
                </div>

                <div class="form-group">
                  <label for="email">
                    Email Address
                  </label>

                  <input
                      id="email"
                      v-model="form.email"
                      type="email"
                      placeholder="example@email.com"
                  />
                </div>

              </div>

              <div class="form-group">
                <label for="address">
                  Business Address
                </label>

                <textarea
                    id="address"
                    v-model="form.address"
                    rows="3"
                    placeholder="Enter complete business address"
                ></textarea>
              </div>

            </div>
          </section>


          <!-- Business Hours -->
          <section class="information-section">

            <div class="section-header">
              <div class="section-icon hours">
                <i class="fa fa-clock-o"></i>
              </div>

              <div>
                <h2>Business Hours</h2>
                <p>Configure your business operating hours</p>
              </div>
            </div>

            <div class="hours-content">

              <!-- Open 7 Days Toggle -->
              <div class="seven-days-row">

                <div>
                  <h3>Open 7 days a week</h3>
                  <p>
                    Your business is available every day of the week.
                  </p>
                </div>

                <label class="switch">
                  <input
                      type="checkbox"
                      v-model="form.openSevenDays"
                      @change="toggleSevenDays"
                  />

                  <span class="slider"></span>
                </label>

              </div>


              <!-- Days -->
              <div class="business-days">

                <div
                    v-for="day in form.businessHours"
                    :key="day.day"
                    class="day-row"
                >

                  <div class="day-name">
                    {{ day.day }}
                  </div>

                  <label class="day-switch">
                    <input
                        type="checkbox"
                        v-model="day.open"
                    />

                    <span class="checkmark"></span>

                    <span class="open-label">
                      {{ day.open ? 'Open' : 'Closed' }}
                    </span>
                  </label>

                  <div
                      v-if="day.open"
                      class="time-fields"
                  >

                    <input
                        type="time"
                        v-model="day.openTime"
                    />

                    <span>to</span>

                    <input
                        type="time"
                        v-model="day.closeTime"
                    />

                  </div>

                </div>

              </div>

            </div>
          </section>


          <!-- Save Actions -->
          <div class="form-actions">

            <button
                type="button"
                class="cancel-button"
                @click="cancelChanges"
            >
              Cancel
            </button>

            <button
                type="submit"
                class="save-button"
            >
              <i class="fa fa-save"></i>
              Save Changes
            </button>

          </div>

        </form>

      </div>
    </div>
  </div>
</template>


<script>
import HeaderComponent from '../component/Header.vue';

export default {
  name: 'BusinessInformation',

  components: {
    HeaderComponent
  },

  data() {
    return {
      form: {
        businessName: 'Ranthila Rent A Car',
        location: 'Embilipitiya',
        businessType: 'Vehicle Rental',

        phone: '',
        email: '',
        address: '',

        openSevenDays: true,

        businessHours: [
          {
            day: 'Monday',
            open: true,
            openTime: '08:00',
            closeTime: '20:00'
          },
          {
            day: 'Tuesday',
            open: true,
            openTime: '08:00',
            closeTime: '20:00'
          },
          {
            day: 'Wednesday',
            open: true,
            openTime: '08:00',
            closeTime: '20:00'
          },
          {
            day: 'Thursday',
            open: true,
            openTime: '08:00',
            closeTime: '20:00'
          },
          {
            day: 'Friday',
            open: true,
            openTime: '08:00',
            closeTime: '20:00'
          },
          {
            day: 'Saturday',
            open: true,
            openTime: '08:00',
            closeTime: '20:00'
          },
          {
            day: 'Sunday',
            open: true,
            openTime: '08:00',
            closeTime: '20:00'
          }
        ]
      }
    };
  },

  methods: {

    toggleSevenDays() {
      this.form.businessHours.forEach(day => {
        day.open = this.form.openSevenDays;
      });
    },

    saveBusinessInformation() {
      /*
       * Save form data to your database here.
       *
       * Example:
       *
       * await dbService.updateBusinessInformation(this.form);
       */

      console.log('Business information:', this.form);

      // Show success message / notification here.
    },

    cancelChanges() {
      this.$router.back();
    },

    handleLogout() {
      this.$emit('logout');
    }
  }
};
</script>


<style scoped>

.business-information-page {
  min-height: 100vh;
  background: var(--color-background-tertiary);
}

.business-information-content {
  padding: 2rem 0;
}

.business-information-container {
  max-width: 1000px;
  margin: 0 auto;
  padding: 0 2rem;
}


/* Page Header */

.page-header {
  margin-bottom: 2rem;
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


/* Information Section */

.information-section {
  margin-bottom: 1.5rem;

  background: var(--color-background-primary);

  border: 1px solid var(--color-border-tertiary);

  border-radius: var(--border-radius-lg);

  overflow: hidden;
}


/* Section Header */

.section-header {
  display: flex;
  align-items: center;

  gap: 1rem;

  padding: 1.25rem 1.5rem;

  border-bottom: 1px solid var(--color-border-tertiary);
}

.section-header h2 {
  margin: 0 0 0.25rem 0;

  font-size: 17px;
  font-weight: 600;

  color: var(--color-text-primary);
}

.section-header p {
  margin: 0;

  font-size: 13px;

  color: var(--color-text-secondary);
}


/* Section Icons */

.section-icon {
  width: 46px;
  height: 46px;

  flex-shrink: 0;

  display: flex;
  align-items: center;
  justify-content: center;

  border-radius: var(--border-radius-md);

  color: white;

  font-size: 20px;
}

.section-icon.business {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.section-icon.contact {
  background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
}

.section-icon.hours {
  background: linear-gradient(135deg, #43e97b 0%, #38f9d7 100%);
}


/* Form */

.form-content {
  padding: 1.5rem;
}

.form-row {
  display: grid;

  grid-template-columns: repeat(2, 1fr);

  gap: 1.25rem;
}

.form-group {
  display: flex;
  flex-direction: column;

  gap: 0.5rem;

  margin-bottom: 1.25rem;
}

.form-group:last-child {
  margin-bottom: 0;
}

.form-group label {
  font-size: 13px;

  font-weight: 500;

  color: var(--color-text-primary);
}

.required {
  color: #dc3545;
}


/* Inputs */

.form-group input,
.form-group textarea {
  width: 100%;

  box-sizing: border-box;

  padding: 0.8rem 0.9rem;

  background: var(--color-background-primary);

  border: 1px solid var(--color-border-tertiary);

  border-radius: var(--border-radius-md);

  color: var(--color-text-primary);

  font-family: inherit;

  font-size: 14px;

  outline: none;

  transition: border-color 0.2s ease,
  box-shadow 0.2s ease;
}

.form-group textarea {
  resize: vertical;
  min-height: 90px;
}

.form-group input:focus,
.form-group textarea:focus {
  border-color: #667eea;

  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}


/* Business Hours */

.hours-content {
  padding: 0 1.5rem 1.5rem;
}


/* Seven Days */

.seven-days-row {
  display: flex;

  align-items: center;
  justify-content: space-between;

  padding: 1.25rem 0;

  border-bottom: 1px solid var(--color-border-tertiary);
}

.seven-days-row h3 {
  margin: 0 0 0.3rem 0;

  font-size: 14px;
  font-weight: 600;

  color: var(--color-text-primary);
}

.seven-days-row p {
  margin: 0;

  font-size: 12px;

  color: var(--color-text-secondary);
}


/* Toggle */

.switch {
  position: relative;

  display: inline-block;

  width: 48px;
  height: 26px;

  flex-shrink: 0;
}

.switch input {
  opacity: 0;
  width: 0;
  height: 0;
}

.slider {
  position: absolute;

  inset: 0;

  cursor: pointer;

  background: #ccc;

  border-radius: 999px;

  transition: 0.2s;
}

.slider:before {
  content: "";

  position: absolute;

  width: 20px;
  height: 20px;

  left: 3px;
  top: 3px;

  background: white;

  border-radius: 50%;

  transition: 0.2s;
}

.switch input:checked + .slider {
  background: #667eea;
}

.switch input:checked + .slider:before {
  transform: translateX(22px);
}


/* Business Days */

.business-days {
  display: flex;
  flex-direction: column;
}

.day-row {
  display: grid;

  grid-template-columns: 130px 130px 1fr;

  align-items: center;

  min-height: 58px;

  border-bottom: 1px solid var(--color-border-tertiary);
}

.day-row:last-child {
  border-bottom: none;
}

.day-name {
  font-size: 14px;

  font-weight: 500;

  color: var(--color-text-primary);
}


/* Day Switch */

.day-switch {
  display: flex;

  align-items: center;

  gap: 0.5rem;

  cursor: pointer;
}

.day-switch input {
  display: none;
}

.checkmark {
  width: 18px;
  height: 18px;

  border: 1px solid var(--color-border-tertiary);

  border-radius: 4px;

  position: relative;
}

.day-switch input:checked + .checkmark {
  background: #667eea;

  border-color: #667eea;
}

.day-switch input:checked + .checkmark:after {
  content: "✓";

  position: absolute;

  left: 3px;
  top: -1px;

  color: white;

  font-size: 13px;
}

.open-label {
  font-size: 13px;

  color: var(--color-text-secondary);
}


/* Time */

.time-fields {
  display: flex;

  align-items: center;

  gap: 0.6rem;
}

.time-fields input {
  padding: 0.55rem 0.65rem;

  background: var(--color-background-primary);

  border: 1px solid var(--color-border-tertiary);

  border-radius: var(--border-radius-md);

  color: var(--color-text-primary);

  font-family: inherit;
}

.time-fields span {
  font-size: 12px;

  color: var(--color-text-secondary);
}


/* Actions */

.form-actions {
  display: flex;

  justify-content: flex-end;

  gap: 0.75rem;

  padding: 0.5rem 0 2rem;
}

.cancel-button,
.save-button {
  display: inline-flex;

  align-items: center;
  justify-content: center;

  gap: 0.5rem;

  padding: 0.75rem 1.25rem;

  border-radius: var(--border-radius-md);

  font-family: inherit;

  font-size: 14px;

  font-weight: 500;

  cursor: pointer;

  transition: all 0.2s ease;
}

.cancel-button {
  background: var(--color-background-primary);

  border: 1px solid var(--color-border-tertiary);

  color: var(--color-text-primary);
}

.cancel-button:hover {
  background: var(--color-background-secondary);
}

.save-button {
  border: none;

  background: linear-gradient(
      135deg,
      #667eea 0%,
      #764ba2 100%
  );

  color: white;

  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.25);
}

.save-button:hover {
  transform: translateY(-1px);

  box-shadow: 0 6px 16px rgba(102, 126, 234, 0.3);
}


/* Mobile */

@media (max-width: 768px) {

  .business-information-content {
    padding: 1.5rem 0;
  }

  .business-information-container {
    padding: 0 1rem;
  }

  .page-header h1 {
    font-size: 24px;
  }

  .form-row {
    grid-template-columns: 1fr;
    gap: 0;
  }

  .day-row {
    grid-template-columns: 1fr;

    gap: 0.75rem;

    padding: 1rem 0;
  }

  .time-fields {
    width: 100%;
  }

  .time-fields input {
    flex: 1;
  }

  .form-actions {
    padding-bottom: 1rem;
  }

  .cancel-button,
  .save-button {
    flex: 1;
  }
}
</style>


