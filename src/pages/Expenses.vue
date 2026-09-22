<template>
  <div class="account-page">
    <HeaderComponent
        :userName="`Logged in as ${loggedUser}`"
        :showBackButton="true"
        @logout="handleLogout"
    />

    <div class="account-content">
      <div class="account-container">

        <!-- Page Header -->
        <div class="page-header">
          <div>
            <h1>Expenses</h1>
            <p class="page-subtitle">Handling daily expenses here</p>
          </div>
          <button class="btn-add-record" @click="openAddExpenseModal">
            <i class="fa fa-plus"></i>
            <span>Add Expense</span>
          </button>
        </div>

        <!-- Expense Records -->
        <div class="table-section">
          <div class="table-header">
            <div>
              <h2 class="section-title">Expense Records</h2>
              <p class="table-subtitle">Add salary, utility bills, maintenance and other expenses.</p>
            </div>

            <div class="table-filters">
              <input
                  v-model="searchQuery"
                  type="text"
                  class="search-input"
                  placeholder="Search expense..."
              />
            </div>
          </div>

          <div class="table-wrapper">
            <table class="account-table">
              <thead>
              <tr>
                <th>Date</th>
                <th>Category</th>
                <th>Description</th>
                <th>Amount</th>
                <th>Payment Status</th>
                <th>Payment Type</th>
                <th>Actions</th>
              </tr>
              </thead>

              <tbody>
              <tr v-for="expense in filteredExpenses" :key="expense.id">
                <td>{{ formatDate(expense.date) }}</td>
                <td><span class="category-badge">{{ expense.category }}</span></td>
                <td>{{ expense.description }}</td>
                <td class="amount-expense">{{ formatPrice(expense.amount) }}</td>
                <td :class="['payment-badge', `payment-${convertSnakeCase(expense.payment_status)}`]">{{ expense.payment_status }}</td>
                <td>{{ expense.payment_type }}</td>
                <td class="actions">
                  <button v-if="expense.payment_status !== paymentStatus.completed" class="btn-action edit" @click="openEditExpenseModal(expense)" title="Edit">
                    <i class="fa fa-edit"></i>
                  </button>
                  <button v-if="expense.payment_status !== paymentStatus.completed" class="btn-action edit" @click="completeExpense(expense.id)" title="Mark Completed">
                    <i class="fa fa-check"></i>
                  </button>
                  <button v-if="isAdmin" class="btn-action delete" @click="deleteExpense(expense.id)" title="Delete" :disabled="!isAdmin">
                    <i class="fa fa-trash"></i>
                  </button>

                </td>
              </tr>
              </tbody>
            </table>

            <div v-if="filteredExpenses.length === 0" class="empty-state">
              <i class="fa fa-receipt"></i>
              <p>No expense records found</p>
              <button class="btn-add-empty" @click="openAddExpenseModal">
                Add Expense
              </button>
            </div>
          </div>
        </div>

      </div>
    </div>

    <!-- Add/Edit Expense Modal -->
    <div v-if="showExpenseModal" class="modal-overlay" @click="closeExpenseModal">
      <div class="modal-content" @click.stop>
        <div class="modal-header">
          <h2>{{ isEditMode ? 'Edit Expense' : 'Add Expense' }}</h2>
          <button class="btn-close" @click="closeExpenseModal">
            <i class="fa fa-times"></i>
          </button>
        </div>

        <div class="modal-body">
          <form @submit.prevent="saveExpense">

            <div class="form-group">
              <label for="expenseCategory">Expense Category <span class="required">*</span></label>
              <select
                  id="expenseCategory"
                  v-model="expenseForm.category"
                  class="input-field"
                  required
              >
                <option value="">Select category</option>
                <option value="Salary">Salary</option>
                <option value="Utility Bill">Utility Bill</option>
                <option value="Vehicle Maintenance">Vehicle Maintenance</option>
                <option value="Rent">Rent</option>
                <option value="Fuel">Fuel</option>
                <option value="Office Expense">Office Expense</option>
                <option value="Other">Other</option>
              </select>
            </div>

            <div class="form-group">
              <label for="expenseDescription">Description <span class="required">*</span></label>
              <input
                  id="expenseDescription"
                  v-model="expenseForm.description"
                  type="text"
                  class="input-field"
                  placeholder="e.g. August staff salaries"
                  required
              />
            </div>

            <div class="form-row">
              <div class="form-group">
                <label for="expenseAmount">Amount (LKR) <span class="required">*</span></label>
                <input
                    id="expenseAmount"
                    v-model.number="expenseForm.amount"
                    type="number"
                    class="input-field"
                    placeholder="0.00"
                    min="0"
                    step="0.01"
                    required
                />
              </div>

              <div class="form-group">
                <label for="expenseDate">Date <span class="required">*</span></label>
                <input
                    id="expenseDate"
                    v-model="expenseForm.date"
                    type="date"
                    class="input-field"
                    required
                />
              </div>
            </div>

            <div class="form-row">
              <div class="form-group">
                <label for="expenseCategory">Payment Type <span class="required">*</span></label>
                <v-select
                    v-model="expenseForm.payment_type"
                    :items="paymentTypeArray"
                    item-title="title"
                    item-value="value"
                    class="input-field"
                    density="compact"
                    variant="outlined"
                    hide-details
                    placeholder="Select payment type"
                />
              </div>

              <div class="form-group">
                <label for="expenseCategory">Payment Status <span class="required">*</span></label>
                <input
                    id="paymentStatus"
                    type="text"
                    :value="expenseForm.payment_status"
                    class="input-field status-field"
                    readonly
                />
              </div>
            </div>
            <div class="form-actions">
              <button type="button" class="btn-secondary" @click="closeExpenseModal">
                Cancel
              </button>
              <button type="submit" class="btn-primary">
                {{ isEditMode ? 'Update Expense' : 'Add Expense' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import HeaderComponent from '../component/Header.vue'
import { dbService } from '../services/db.ts'
import { useSnackbar } from '../composables/useSnackbar'
import { useAuthStore } from '../stores/auth.ts'
import {
  paymentStatus,
  roleTypes,
  convertSnakeCase,
  paymentTypes, paymentTypeArray
} from '../utils/constants.ts'

const emit = defineEmits(['logout'])

const router = useRouter()
const { showSuccess, showError } = useSnackbar()
const authStore = useAuthStore()

// Reactive State
const loggedUser = ref(authStore.username)
const roleName = ref(authStore.role)
const expenseRecords = ref([])
const searchQuery = ref('')
const showExpenseModal = ref(false)
const isEditMode = ref(false)
const editingExpenseId = ref(null)

const expenseForm = ref({
  category: '',
  description: '',
  amount: 0,
  date: '',
  payment_status: '',
  payment_type: ''
})

// Computed Properties
const isAdmin = computed(() => roleName.value === roleTypes.admin)

const filteredExpenses = computed(() => {
  if (!searchQuery.value) return expenseRecords.value

  const query = searchQuery.value.toLowerCase()

  return expenseRecords.value.filter(item =>
      String(item.category || '').toLowerCase().includes(query) ||
      String(item.description || '').toLowerCase().includes(query) ||
      String(item.notes || '').toLowerCase().includes(query)
  )
})

// Watchers
watch(
    () => expenseForm.value.payment_type,
    (newValue) => {
      if (newValue === paymentTypes.cashPayment || newValue === paymentTypes.cardPayment || newValue === paymentTypes.bankTransfer ) {
        expenseForm.value.payment_status = paymentStatus.completed
      } else if (newValue === paymentTypes.creditPayment ) {
        expenseForm.value.payment_status = paymentStatus.pending
      } else {
        expenseForm.value.payment_status = ''
      }
    }
)

const formatPrice = (price) => {
  return new Intl.NumberFormat('en-US', {
    style: 'currency',
    currency: 'LKR',
    minimumFractionDigits: 2,
    maximumFractionDigits: 2
  }).format(Number(price || 0))
}

const formatDate = (dateString) => {
  if (!dateString) return '-'

  return new Date(dateString).toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  })
}

const openAddExpenseModal = () => {
  isEditMode.value = false
  editingExpenseId.value = null

  expenseForm.value = {
    category: '',
    description: '',
    amount: 0,
    date: new Date().toISOString().slice(0, 10)
  }

  showExpenseModal.value = true
}

const openEditExpenseModal = (expense) => {
  isEditMode.value = true
  editingExpenseId.value = expense.id

  expenseForm.value = {
    category: expense.category || '',
    description: expense.description || '',
    amount: Number(expense.amount || 0),
    date: expense.date || '',
    payment_type: expense.payment_type || '',
    payment_status: expense.payment_status || ''
  }

  showExpenseModal.value = true
}

const closeExpenseModal = () => {
  showExpenseModal.value = false
  editingExpenseId.value = null
}

const saveExpense = async () => {
  if (
      !expenseForm.value.category ||
      !expenseForm.value.description ||
      !expenseForm.value.amount ||
      !expenseForm.value.date
  ) {
    alert('Please fill all required fields')
    return
  }

  const record = {
    category: expenseForm.value.category,
    description: expenseForm.value.description,
    amount: Number(expenseForm.value.amount),
    date: expenseForm.value.date,
    payment_status: expenseForm.value.payment_status,
    payment_type: expenseForm.value.payment_type,
    created_by: loggedUser.value
  }

  try {
    if (isEditMode.value) {
      const updateRequest = {
        id: editingExpenseId.value,
        category: expenseForm.value.category,
        description: expenseForm.value.description,
        amount: Number(expenseForm.value.amount),
        date: expenseForm.value.date,
        payment_status: expenseForm.value.payment_status,
        payment_type: expenseForm.value.payment_type
      }

      await dbService.updateExpenses(updateRequest)

      const index = expenseRecords.value.findIndex(
          item => item.id === editingExpenseId.value
      )

      if (index !== -1) {
        expenseRecords.value.splice(index, 1, record)
      }
    } else {
      await dbService.addExpenses(record)
      expenseRecords.value.unshift(record)
    }

    showSuccess(isEditMode.value ? 'Expense updated successfully!' : 'Expense added successfully!')
    closeExpenseModal()
  } catch (error) {
    console.error('Error saving expense:', error)
    showError('Error saving expense. Please check the account database/service.')
  }
}

const deleteExpense = async (id) => {
  try {
    await dbService.deleteExpense(id)

    const index = expenseRecords.value.findIndex(item => item.id === id)

    if (index !== -1) {
      expenseRecords.value.splice(index, 1)
    }

    showSuccess('Expense deleted successfully!')
  } catch (error) {
    console.error('Error deleting expense:', error)
    showError('Error deleting expense. Please try again.')
  }
}

const completeExpense = async (id) => {
  try {
    await dbService.completeExpense({
      id: id,
      payment_status: paymentStatus.completed
    })

    await getExpenseRecords()
  } catch (error) {
    console.error('Error completing expense:', error)
    showError('Error completing expense. Please try again.')
  }
}

const getExpenseRecords = async () => {
  try {
    const result = await dbService.getExpenses()

    expenseRecords.value = result.map(item => ({
      id: item.id,
      name: item.name,
      category: item.category,
      description: item.description,
      amount: Number(item.amount || 0),
      date: item.date || item.date,
      payment_type: item.payment_type,
      payment_status: item.payment_status,
      createdDate: item.created_at,
      createdBy: item.created_by
    }))
  } catch (error) {
    console.error('Error fetching expense records:', error)
    expenseRecords.value = []
  }
}

const handleLogout = () => {
  emit('logout')
}

// Lifecycle
onMounted(async () => {
  await Promise.all([getExpenseRecords()])
})
</script>

<style scoped>
.account-page {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
  background: var(--color-background-tertiary);
}

.account-content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.account-container {
  flex: 1;
  overflow-y: auto;
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
  font-weight: 700;
  color: var(--color-text-primary);
}

.page-subtitle,
.table-subtitle {
  margin: 0.35rem 0 0;
  font-size: 13px;
  color: var(--color-text-secondary);
}

.btn-add-record,
.btn-primary {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.6rem;
  padding: 0.75rem 1.35rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: var(--border-radius-md);
  cursor: pointer;
  font-size: 14px;
  font-weight: 600;
}

.stats-section {
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.stat-card {
  background: var(--color-background-primary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-lg);
  padding: 1.25rem;
  display: flex;
  align-items: center;
  gap: 1rem;
  min-width: 0;
}

.stat-icon {
  width: 44px;
  height: 44px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--color-background-secondary);
  color: var(--color-info);
  flex-shrink: 0;
}

.income-card .stat-icon {
  color: var(--color-success);
}

.expense-card .stat-icon {
  color: var(--color-danger);
}

.balance-card .stat-icon {
  color: #764ba2;
}

.stat-card label {
  display: block;
  font-size: 12px;
  color: var(--color-text-secondary);
  font-weight: 600;
  margin-bottom: 0.4rem;
}

.stat-value {
  display: block;
  font-size: 21px;
  font-weight: 700;
  color: var(--color-text-primary);
  white-space: nowrap;
}

.charts-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
  margin-bottom: 1rem;
}

.chart-card {
  background: var(--color-background-primary);
  border: 1px solid var(--color-border-tertiary);
  border-radius: var(--border-radius-lg);
  padding: 1.25rem;
  min-width: 0;
}

.combined-chart {
  margin-bottom: 1.5rem;
}

.chart-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 1rem;
  margin-bottom: 0.5rem;
}

.chart-header h2 {
  margin: 0;
  font-size: 16px;
  color: var(--color-text-primary);
}

.chart-header p {
  margin: 0.3rem 0 0;
  font-size: 12px;
  color: var(--color-text-secondary);
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
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 1rem;
}

.section-title {
  margin: 0;
  font-size: 16px;
  color: var(--color-text-primary);
}

.table-filters {
  width: 280px;
}

.search-input {
  width: 100%;
  padding: 0.65rem 0.9rem;
  border: 1px solid var(--color-border-tertiary);
  border-radius: 6px;
  font-size: 13px;
  color: var(--color-text-primary);
  background: var(--color-background-secondary);
}

.search-input:focus,
.input-field:focus {
  outline: none;
  border-color: var(--color-info);
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.table-wrapper {
  overflow-x: auto;
}

.account-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 13px;
}

.account-table th {
  padding: 0.8rem;
  text-align: left;
  color: var(--color-text-secondary);
  background: var(--color-background-secondary);
  font-weight: 600;
}

.account-table td {
  padding: 0.8rem;
  border-bottom: 1px solid var(--color-border-tertiary);
  color: var(--color-text-primary);
}

.amount-expense {
  color: var(--color-danger);
  font-family: monospace;
  font-weight: 700;
  text-align: left;
}

.category-badge {
  display: inline-block;
  padding: 0.3rem 0.65rem;
  border-radius: 12px;
  background: var(--color-background-secondary);
  color: var(--color-text-primary);
  font-size: 11px;
  font-weight: 600;
}

.actions {
  display: flex;
  gap: 0.4rem;
}

.btn-action {
  padding: 0.4rem 0.5rem;
  border: 1px solid var(--color-border-tertiary);
  background: transparent;
  color: var(--color-text-secondary);
  border-radius: 4px;
  cursor: pointer;
}

.btn-action.edit:hover {
  color: #f59e0b;
  border-color: #f59e0b;
}

.btn-action.delete:hover {
  color: var(--color-danger);
  border-color: var(--color-danger);
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
  font-size: 48px;
  margin-bottom: 1rem;
  opacity: 0.3;
}

.btn-add-empty {
  padding: 0.7rem 1.25rem;
  border: none;
  border-radius: var(--border-radius-md);
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  cursor: pointer;
  font-weight: 600;
}

.modal-overlay {
  position: fixed;
  inset: 0;
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
  max-width: 650px;
  max-height: 90vh;
  overflow-y: auto;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
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

.modal-header h2 {
  margin: 0;
  font-size: 19px;
}

.btn-close {
  border: none;
  background: transparent;
  color: white;
  cursor: pointer;
  font-size: 19px;
}

.modal-body {
  padding: 1.5rem;
}

.form-group {
  margin-bottom: 1.25rem;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 1rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.55rem;
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-primary);
}

.required {
  color: var(--color-danger);
}

.input-field {
  width: 100%;
  box-sizing: border-box;
  padding: 0.75rem;
  border: 1px solid var(--color-border-tertiary);
  border-radius: 5px;
  font-size: 14px;
  color: var(--color-text-primary);
  background: var(--color-background-primary);
}

.textarea {
  resize: vertical;
  min-height: 80px;
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 0.75rem;
  padding-top: 1.25rem;
  border-top: 1px solid var(--color-border-tertiary);
}

.btn-secondary {
  padding: 0.75rem 1.25rem;
  border: 1px solid var(--color-border-tertiary);
  background: transparent;
  color: var(--color-text-primary);
  border-radius: var(--border-radius-md);
  cursor: pointer;
  font-weight: 600;
}

.payment-badge {
  display: inline-block;
  padding: 0.4rem 0.8rem;
  border-radius: 20px;
  font-size: 11px;
  font-weight: 500;
}

.payment-completed {
  background: #dcfce7;
  color: #16a34a;
}

.payment-pending {
  background: #fef3c7;
  color: #d97706;
}


@media (max-width: 1024px) {
  .stats-section {
    grid-template-columns: repeat(2, 1fr);
  }

  .charts-grid {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 768px) {
  .account-container {
    padding: 1rem;
  }

  .page-header {
    flex-direction: column;
    align-items: flex-start;
  }

  .btn-add-record {
    width: 100%;
  }

  .stats-section {
    grid-template-columns: 1fr;
  }

  .table-header {
    flex-direction: column;
    align-items: stretch;
  }

  .table-filters {
    width: 100%;
  }

  .form-row {
    grid-template-columns: 1fr;
  }

  .form-actions {
    flex-direction: column;
  }

  .form-actions button {
    width: 100%;
  }
}
</style>
