import { createRouter, createWebHashHistory } from 'vue-router'
import Dashboard from "../pages/Dashboard.vue";
import MakeOrder from "../pages/MakeOrder.vue";
import ViewOrder from "../pages/ViewOrder.vue";
import Products from "../pages/ManageProducts.vue";
import Account from "../pages/Account.vue";
import SystemUserManagement from "../pages/SystemUserManagement.vue";
import Scheduler from "../pages/Scheduler.vue";
import Maintenance from "../pages/Maintenance.vue";
import IncidentHandling from "../pages/IncidentHandling.vue";
import Analysing from "../pages/Analysing.vue";
import Gallery from "../pages/Gallery.vue";
import Expenses from "../pages/Expenses.vue";
import Settings from "../pages/Settings.vue";
import BusinessInformation from "../pages/BusinessInformation.vue";
import SystemInformation from "../pages/SystemInformation.vue";
import SummeryOverview from "../pages/SummeryOverview.vue";
import SummeryYearly from "../pages/SummeryYearly.vue";
import SummeryVehicles from "../pages/SummeryVehicles.vue";
import OrderDetails from "../pages/OrderDetails.vue";

const routes = [
    { path: '/', component: Dashboard },
    { path: '/orders', component: MakeOrder },
    { path: '/view-order', component: ViewOrder },
    { path: '/products', component: Products },
    { path: '/account', component: Account },
    { path: '/Settings', component: Settings },
    { path: '/scheduler', component: Scheduler },
    { path: '/maintenance', component: Maintenance },
    { path: '/incident-handling', component: IncidentHandling },
    { path: '/analysing', component: Analysing },
    { path: '/gallery', component: Gallery },
    { path: '/expenses', component: Expenses },
    { path: '/settings/system-users', component: SystemUserManagement },
    { path: '/settings/business-information', component: BusinessInformation },
    { path: '/settings/system-information', component: SystemInformation },
    { path: '/summery-overview', component: SummeryOverview},
    { path: '/summery-yearly', component: SummeryYearly},
    { path: '/summery-vehicle', component: SummeryVehicles},
    { path: '/view-order-detail/:id', name: 'view-order-detail', component: OrderDetails},
]

const index = createRouter({
    history: createWebHashHistory(), // IMPORTANT for Electron
    routes
})

export default index
