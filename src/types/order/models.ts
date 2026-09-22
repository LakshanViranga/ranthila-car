import {OrderItem} from "../../services/db";

export interface Order {
    id: number;
    order_number: string;
    customer_id?: number;
    subtotal: number;
    service_charge: number;
    discount_amount: number;
    total_amount: number;
    payment_method: string;
    payment_status: string;
    order_status: string;
    items: OrderItem[];
    notes?: string;
}

export interface CreateOrderRequest {
    order_number: string,
    customer_id?: string,
    items: Array<{ product_id: number; quantity: number, unit_price: number, item_total: number }>,
    subtotal: number,
    service_charge: number,
    discount_amount: number,
    total_amount: number,
    payment_method: string,
    order_type: string,
    notes: string,
    order_status: string,
}

export interface UpdateOrderStatusRequest {
    id: number;
    order_status: string;
}
