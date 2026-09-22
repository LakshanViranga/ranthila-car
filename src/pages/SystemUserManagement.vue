<template>
  <div class="settings-page">
    <!-- Header Component -->
    <header-component
        :userName="`Logged in as ${loggedUser}`"
        :showBackButton="true"
        @logout="handleLogout"
    />

    <!-- Settings Content -->
    <div class="settings-content">
      <div class="settings-container">

        <!-- Page Header -->
        <div class="page-header">
          <div>
            <h1 v-text="'User Management'"></h1>
            <p class="page-subtitle" v-text="'Create, edit, and manage system users'"></p>
          </div>
        </div>

        <!-- User Management Section -->
        <div class="user-management-section">

          <!-- Section Header -->
          <div class="section-header">
            <div>
              <h2 class="section-title" v-text="'Users'"></h2>
              <p class="section-subtitle" v-text="`Total users: ${users.length}`"></p>
            </div>
            <button class="btn-add-user" @click="openCreateUserModal">
              <i class="fa fa-plus"></i>
              <span v-text="'Add New User'"></span>
            </button>
          </div>

          <!-- Search and Filter -->
          <div class="search-bar">
            <input
                v-model="searchQuery"
                type="text"
                class="search-input"
                placeholder="Search users by name, email, or role..."
            />
            <select v-model="filterRole" class="role-filter">
              <option value="">All Roles</option>
              <option v-for="role in roleOptions" :key="role" :value="role">
                {{ role }}
              </option>
            </select>
          </div>

          <!-- Users Table -->
          <div class="table-wrapper">
            <table v-if="filteredUsers.length > 0" class="users-table">
              <thead>
              <tr>
                <th v-text="'Username'"></th>
                <th v-text="'Role'"></th>
                <th v-text="'Status'"></th>
                <th v-text="'Joined'"></th>
                <th v-text="'Actions'"></th>
              </tr>
              </thead>
              <tbody>
              <tr v-for="user in filteredUsers" :key="user.id">
                <td class="name-cell">
                  <div class="user-avatar">
                    {{ user.username }}
                  </div>
                  <span v-text="user.username"></span>
                </td>
                <td>
                  <span class="role-badge" :class="`role-${user.role.toLowerCase().replace(' ', '-')}`">
                    {{ user.role }}
                  </span>
                </td>
                <td>
                  <span
                      class="status-badge"
                      :class="{ 'status-active': user.status === 'Active', 'status-inactive': user.status === 'Inactive' }"
                  >
                    {{ user.status }}
                  </span>
                </td>
                <td v-text="formatDate(user.created_at)"></td>
                <td class="actions">
                  <button class="btn-action edit" @click="openEditUserModal(user)" title="Edit">
                    <i class="fa fa-edit"></i>
                  </button>
                  <button class="btn-action delete" @click="deleteUser(user.id)" title="Delete">
                    <i class="fa fa-trash"></i>
                  </button>
                </td>
              </tr>
              </tbody>
            </table>

            <div v-else class="empty-state">
              <i class="fa fa-users"></i>
              <p v-text="'No users found'"></p>
            </div>
          </div>

        </div>

      </div>
    </div>

    <!-- Create/Edit User Modal -->
    <div v-if="showUserModal" class="modal-overlay" @click="closeUserModal">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h2 v-text="isEditingUser ? 'Edit User' : 'Create New User'"></h2>
          <button class="btn-close" @click="closeUserModal">
            <i class="fa fa-times"></i>
          </button>
        </div>

        <div class="modal-body">
          <form @submit.prevent="saveUser">
            <!-- Name -->
            <div class="form-group">
              <label class="form-label">User Name <span class="required">*</span></label>
              <input
                  v-model="userForm.username"
                  type="text"
                  class="form-input"
                  placeholder="Enter full name"
                  required
              />
            </div>

            <!-- Role -->
            <div class="form-group">
              <label class="form-label">Role <span class="required">*</span></label>
              <select v-model="userForm.role" class="form-input" required>
                <option value="">Select a role</option>
                <option v-for="role in roleOptions" :key="role" :value="role">
                  {{ role }}
                </option>
              </select>
            </div>

            <!-- Password (only for new users) -->
            <div v-if="!isEditingUser" class="form-group">
              <label class="form-label">Password <span class="required">*</span></label>
              <input
                  v-model="userForm.password"
                  type="password"
                  class="form-input"
                  placeholder="Enter password"
                  required
              />
              <p class="form-hint" v-text="'Password must be at least 6 characters'"></p>
            </div>

            <!-- Status -->
            <div class="form-group">
              <label class="form-label">Status <span class="required">*</span></label>
              <select v-model="userForm.status" class="form-input" required>
                <option :value="userStatus.ACTIVE">Active</option>
                <option :value="userStatus.INACTIVE">Inactive</option>
              </select>
            </div>

            <!-- Form Actions -->
            <div class="form-actions">
              <button type="button" class="btn-secondary" @click="closeUserModal">
                <i class="fa fa-times"></i>
                <span v-text="'Cancel'"></span>
              </button>
              <button type="submit" class="btn-primary">
                <i class="fa fa-save"></i>
                <span v-text="isEditingUser ? 'Update User' : 'Create User'"></span>
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
import {userStatus} from "../utils/constants.ts";

export default {
  name: 'UserManagement',
  components: {
    HeaderComponent
  },
  data() {
    return {
      loggedUser: 'Admin User',

      // Users Management
      users: [],
      searchQuery: '',
      filterRole: '',
      showUserModal: false,
      isEditingUser: false,
      userForm: {
        username: '',
        role: '',
        password: '',
        status: userStatus.ACTIVE
      },

      // Roles
      roleOptions: ['Admin', 'Manager', 'Staff', 'Customer Service']
    };
  },

  computed: {
    userStatus() {
      return userStatus
    },
    filteredUsers() {
      return this.users.filter(user => {
        const matchesSearch = user.username.toLowerCase().includes(this.searchQuery.toLowerCase()) ||
            user.email.toLowerCase().includes(this.searchQuery.toLowerCase()) ||
            user.role.toLowerCase().includes(this.searchQuery.toLowerCase());

        const matchesRole = !this.filterRole || user.role === this.filterRole;

        return matchesSearch && matchesRole;
      });
    }
  },

  methods: {
    /**
     * Format date to readable format
     */
    formatDate(dateString) {
      if (!dateString) return '-';
      return new Date(dateString).toLocaleDateString('en-US', {
        year: 'numeric',
        month: 'short',
        day: 'numeric'
      });
    },

    /**
     * Open create user modal
     */
    openCreateUserModal() {
      this.isEditingUser = false;
      this.userForm = {
        username: '',
        role: '',
        password: '',
        status: userStatus.ACTIVE
      };
      this.showUserModal = true;
    },

    /**
     * Open edit user modal
     */
    openEditUserModal(user) {
      this.isEditingUser = true;
      this.userForm = {
        ...user,
        password: ''
      };
      this.showUserModal = true;
    },

    /**
     * Close user modal
     */
    closeUserModal() {
      this.showUserModal = false;
      this.userForm = {
        username: '',
        role: '',
        password: '',
        status: userStatus.ACTIVE
      };
    },

    /**
     * Save user (create or update)
     */
    async saveUser() {
      if (!this.userForm.username || !this.userForm.role) {
        alert('Please fill in all required fields');
        return;
      }

      if (!this.isEditingUser && !this.userForm.password) {
        alert('Password is required for new users');
        return;
      }

      if (!this.isEditingUser && this.userForm.password.length < 6) {
        alert('Password must be at least 6 characters');
        return;
      }

      try {
        if (this.isEditingUser) {
          // Update existing user
          const index = this.users.findIndex(u => u.id === this.userForm.id);
          if (index !== -1) {
            this.users.splice(index, 1, { ...this.userForm });
          }
          alert('User updated successfully!');
        } else {
          // Create new user
          const newUser = {
            ...this.userForm,
          };
          console.log(newUser);
          await dbService.saveUser(newUser);

          this.users.push(newUser);
          alert('User created successfully!');
        }
        this.closeUserModal();
      } catch (error) {
        console.error('Error saving user:', error);
        alert('Error saving user. Please try again.');
      }
    },

    /**
     * Delete user
     */
    deleteUser(userId) {
      if (!confirm('Are you sure you want to delete this user?')) return;

      try {
        const index = this.users.findIndex(u => u.id === userId);
        if (index !== -1) {
          this.users.splice(index, 1);
        }
        alert('User deleted successfully!');
      } catch (error) {
        console.error('Error deleting user:', error);
        alert('Error deleting user. Please try again.');
      }
    },

    /**
     * Handle logout
     */
    handleLogout() {
      this.$emit('logout');
    },

    /**
     * Load users on mount
     */
    async loadUsers() {
      try {
        this.users = await dbService.getUsers();
      } catch (error) {
        console.error('Error loading users:', error);
      }
    }
  },

  mounted() {
    this.loadUsers();
  }
};
</script>

<style scoped>
/* === GLOBAL STYLES === */
:root {
  --transition-fast: 0.2s ease;
  --transition-normal: 0.3s ease;
  --border-radius-lg: 12px;
  --border-radius-md: 8px;
}

/* === PAGE LAYOUT === */
.settings-page {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
  background: linear-gradient(135deg, var(--color-background-tertiary) 0%, var(--color-background-primary) 100%);
}

.settings-content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.settings-container {
  flex: 1;
  overflow-y: auto;
  overflow-x: hidden;
  padding: 2rem;
  max-width: 1400px;
  margin: 0 auto;
  width: 100%;
  box-sizing: border-box;
  scroll-behavior: smooth;
}

.settings-container::-webkit-scrollbar {
  width: 6px;
}

.settings-container::-webkit-scrollbar-track {
  background: transparent;
}

.settings-container::-webkit-scrollbar-thumb {
  background: var(--color-border-secondary);
  border-radius: 3px;
}

.settings-container::-webkit-scrollbar-thumb:hover {
  background: var(--color-border-primary);
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

/* === USER MANAGEMENT SECTION === */
.user-management-section {
  background: var(--color-background-primary);
  border-radius: var(--border-radius-lg);
  border: 1px solid var(--color-border-tertiary);
  overflow: hidden;
}

/* === SECTION HEADER === */
.section-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  gap: 1.5rem;
  padding: 1.75rem;
  border-bottom: 1px solid var(--color-border-tertiary);
}

.section-title {
  margin: 0;
  font-size: 18px;
  font-weight: 700;
  color: var(--color-text-primary);
}

.section-subtitle {
  margin: 0.35rem 0 0;
  font-size: 12px;
  color: var(--color-text-secondary);
}

/* === BUTTONS === */
.btn-add-user {
  display: flex;
  align-items: center;
  gap: 0.6rem;
  padding: 0.75rem 1.5rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: var(--border-radius-md);
  cursor: pointer;
  font-weight: 600;
  font-size: 13px;
  transition: all var(--transition-normal);
  white-space: nowrap;
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
}

.btn-add-user:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 16px rgba(102, 126, 234, 0.4);
}

/* === SEARCH BAR === */
.search-bar {
  display: flex;
  gap: 1rem;
  padding: 1rem 1.75rem;
  border-bottom: 1px solid var(--color-border-tertiary);
}

.search-input {
  flex: 1;
  padding: 0.75rem 1rem;
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-md);
  font-size: 13px;
  color: var(--color-text-primary);
  background: var(--color-background-primary);
  transition: all var(--transition-fast);
}

.search-input:focus {
  outline: none;
  border-color: #667eea;
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}

.role-filter {
  min-width: 150px;
  padding: 0.75rem 1rem;
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-md);
  font-size: 13px;
  color: var(--color-text-primary);
  background: var(--color-background-primary);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.role-filter:focus {
  outline: none;
  border-color: #667eea;
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}

/* === TABLE === */
.table-wrapper {
  overflow-x: auto;
}

.users-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

.users-table th {
  padding: 1rem 0.75rem;
  text-align: left;
  color: var(--color-text-secondary);
  background: var(--color-background-secondary);
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  border-bottom: 2px solid var(--color-border-tertiary);
}

.users-table td {
  padding: 1rem 0.75rem;
  border-bottom: 1px solid var(--color-border-tertiary);
  color: var(--color-text-primary);
}

.users-table tbody tr:hover {
  background: var(--color-background-secondary);
}

.users-table tbody tr:last-child td {
  border-bottom: none;
}

.name-cell {
  display: flex;
  align-items: center;
  gap: 0.85rem;
  font-weight: 600;
}

.user-avatar {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border-radius: 50%;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  font-weight: 700;
  font-size: 12px;
  flex-shrink: 0;
}

.role-badge {
  display: inline-block;
  padding: 0.4rem 0.75rem;
  border-radius: 6px;
  font-size: 11px;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.3px;
}

.role-badge.role-admin {
  background: rgba(239, 68, 68, 0.15);
  color: #dc2626;
}

.role-badge.role-manager {
  background: rgba(245, 158, 11, 0.15);
  color: #b45309;
}

.role-badge.role-staff {
  background: rgba(34, 197, 94, 0.15);
  color: #15803d;
}

.role-badge.role-customer-service {
  background: rgba(59, 130, 246, 0.15);
  color: #1e40af;
}

.status-badge {
  display: inline-block;
  padding: 0.4rem 0.75rem;
  border-radius: 6px;
  font-size: 11px;
  font-weight: 700;
}

.status-badge.status-active {
  background: rgba(34, 197, 94, 0.15);
  color: #15803d;
}

.status-badge.status-inactive {
  background: rgba(107, 114, 128, 0.15);
  color: #374151;
}

.actions {
  display: flex;
  gap: 0.5rem;
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
  font-size: 13px;
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
  padding: 3rem 2rem;
  color: var(--color-text-secondary);
}

.empty-state i {
  font-size: 48px;
  margin-bottom: 1rem;
  opacity: 0.2;
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
  max-width: 500px;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
  animation: slideUp 0.3s ease;
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
  padding: 1.5rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border-radius: var(--border-radius-lg) var(--border-radius-lg) 0 0;
  gap: 1rem;
}

.modal-header h2 {
  margin: 0;
  font-size: 17px;
  font-weight: 700;
}

.btn-close {
  padding: 0.4rem;
  border: none;
  background: transparent;
  color: white;
  cursor: pointer;
  font-size: 18px;
  transition: color var(--transition-fast);
  display: flex;
  align-items: center;
  justify-content: center;
}

.btn-close:hover {
  color: rgba(255, 255, 255, 0.8);
}

.modal-body {
  padding: 1.75rem;
}

.form-group {
  margin-bottom: 1.25rem;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.form-label {
  font-size: 12px;
  font-weight: 700;
  color: var(--color-text-primary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  margin: 0;
}

.required {
  color: #ef4444;
  font-weight: 700;
}

.form-input {
  padding: 0.75rem;
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-md);
  font-size: 13px;
  color: var(--color-text-primary);
  background: var(--color-background-primary);
  transition: all var(--transition-fast);
  font-family: inherit;
}

.form-input:focus {
  outline: none;
  border-color: #667eea;
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}

.form-hint {
  margin: 0.25rem 0 0;
  font-size: 11px;
  color: var(--color-text-secondary);
  font-style: italic;
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
  margin-top: 1.75rem;
  padding-top: 1.5rem;
  border-top: 1px solid var(--color-border-tertiary);
}

.btn-primary,
.btn-secondary {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem 1.25rem;
  border: none;
  border-radius: var(--border-radius-md);
  cursor: pointer;
  font-weight: 600;
  font-size: 13px;
  transition: all var(--transition-normal);
}

.btn-primary {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.3);
}

.btn-primary:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 16px rgba(102, 126, 234, 0.4);
}

.btn-secondary {
  background: transparent;
  color: var(--color-text-primary);
  border: 1px solid var(--color-border-tertiary);
}

.btn-secondary:hover {
  background: var(--color-background-secondary);
  border-color: #667eea;
  color: #667eea;
}

/* === RESPONSIVE === */
@media (max-width: 768px) {
  .settings-container {
    padding: 1rem;
  }

  .page-header h1 {
    font-size: 24px;
  }

  .section-header {
    flex-direction: column;
    gap: 1rem;
    padding: 1.25rem;
  }

  .btn-add-user {
    width: 100%;
    justify-content: center;
  }

  .search-bar {
    flex-direction: column;
    padding: 1rem;
  }

  .role-filter {
    width: 100%;
  }

  .users-table th,
  .users-table td {
    padding: 0.75rem 0.5rem;
    font-size: 12px;
  }

  .user-avatar {
    width: 32px;
    height: 32px;
    font-size: 11px;
  }

  .role-badge,
  .status-badge {
    font-size: 10px;
    padding: 0.3rem 0.6rem;
  }

  .modal-content {
    width: 95%;
    max-width: 100%;
  }

  .form-input {
    font-size: 12px;
  }

  .modal-header h2 {
    font-size: 15px;
  }
}

@media (max-width: 480px) {
  .settings-container {
    padding: 0.75rem;
  }

  .page-header h1 {
    font-size: 20px;
  }

  .section-title {
    font-size: 16px;
  }

  .section-header {
    padding: 1rem;
  }

  .search-bar {
    padding: 0.75rem;
    gap: 0.5rem;
  }

  .users-table {
    font-size: 11px;
  }

  .users-table th,
  .users-table td {
    padding: 0.6rem 0.4rem;
  }

  .name-cell {
    gap: 0.5rem;
  }

  .user-avatar {
    width: 28px;
    height: 28px;
    font-size: 10px;
  }

  .actions {
    gap: 0.3rem;
  }

  .btn-action {
    padding: 0.3rem 0.4rem;
    font-size: 11px;
  }

  .modal-header h2 {
    font-size: 14px;
  }

  .form-label {
    font-size: 11px;
  }

  .form-input {
    font-size: 11px;
    padding: 0.6rem;
  }
}
</style>
