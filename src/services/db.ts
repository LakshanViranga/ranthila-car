import { invoke } from '@tauri-apps/api/core';
import type {CreateOrderRequest, Order, UpdateOrderStatusRequest} from '../types/order/models';
import camelcaseKeys from "camelcase-keys";

export interface OrderItem {
    id?: number;
    order_id?: number;
    product_id: number;
    quantity: number;
    unit_price: number;
    item_total: number;
    notes?: string;
}

export const dbService = {
    // Orders
    async createOrder(request: any) {
        return invoke<Order>('create_order', { request });
    },

    async getAllOrders() {
        const orders = await  invoke<Order[]>('get_orders', {});
        return camelcaseKeys(orders, {deep: true});
    },

    async updateOrder(request: any) {
        return invoke<Order>('update_order', { request });
    },

    async deleteOrder(deleteId: string) {
        return invoke('delete_order', { id: deleteId });
    },

    async getOrdersByDate(date: string) {
        return invoke<Order[]>('get_orders_by_date', { date });
    },

    // Vehicle
    async createVehicle(request: any) {
        return invoke('create_vehicle', { request });
    },
    async updateVehicle(request: any) {
        return invoke('update_vehicle', { request });
    },
    async getVehicles() {
        return invoke<any>('get_vehicles');
    },
    async deleteVehicle(id: string) {
        return invoke('delete_vehicle', { vehicleId: id });
    },

    async getOrderId() {
        return invoke<String>('get_order_sequence');
    },

    // Customer related
    async createCustomer(request: any){
        return invoke('create_customer', { request });
    },
    async getCustomerByIdentity(customerId: string) {
        return invoke<any>('get_customer_by_identity_number', { nationalId: customerId });
    },

    // Maintenance
    async addMaintenance(request: any){
        return invoke('add_maintenance_record', { request });
    },
    async getMaintenanceRecords() {
        return invoke<any>('get_maintenance_record');
    },

    async updateMaintenanceRecords(request: any){
        return invoke<any>('update_maintenance', { request });
    },
    async deleteMaintenanceRecords(id: string) {
        return invoke<any>('delete_maintenance', { id });
    },
    async completeMaintenance(request: any){
        return invoke<any>('complete_maintenance', { request });
    },
    // Expenses
    async addExpenses(request: any){
        return invoke('add_expenses', { request });
    },
    async getExpenses() {
        return invoke<any>('get_expenses');
    },
    async updateExpenses(request: any){
        return invoke('update_expenses', { request });
    },
    async deleteExpense(id: string) {
        return invoke('delete_expenses', { id });
    },
    async completeExpense(request: any){
        return invoke('complete_expenses', { request });
    },

    // Account Summery
    async addAccountSummery(request: any){
        return invoke('add_account_summery', { request });
    },
    async getAccountSummery(){
        return invoke<any>('get_account_summery');
    },

    // Incident Management
    async getCustomerOrders(id: string) {
        return invoke<any>('get_customer_order', { nationalId: id });
    },

    async getIncidents(id: string) {
        return invoke<any>('get_incidents', { nationalId: id });
    },

    async addIncident(request: any){
        return invoke('add_incident', { request });
    },

    async getOrderByOrderNumber(id: string) {
        return invoke<any>('get_order_by_order_number', { orderId: id });
    },

    // Transactions
    async addTransaction(request: any){
        return invoke('add_transaction', { request });
    },

    async getTransactions() {
        return invoke<any>('get_transaction');
    },

    // Login request
    async getUsers(){
        return invoke<any>('get_users');
    },

    async saveUser(request: any){
        return invoke('save_user', { request });
    },

    async login(request: any){
        return invoke('login', { request });
    },

    // Restriction add
    async addRestrictedCustomers(request: any){
        return invoke('add_restricted_customer', { request });
    },

    async removeRestrictedCustomer(id: string) {
        return invoke('remove_restricted_customer', { nic: id });
    },

    //Filter orders
    async filterOrder(params: string){
        return invoke('filter_order', { request: params });
    },

    // Bank records
    async addBankRecords(request: any){
        return invoke('add_bank_record', { request });
    },

    async getBankRecords(request: any){
        return invoke('get_all_bank_records', { request });
    },

    // Update Vehicle Mileage
    async updateVehicleMilage(request: any){
        return invoke('update_vehicle_mileage', { request });
    }
};
