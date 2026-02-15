import { App } from "./components/App";

const baseUrl = import.meta.env.VITE_ECONET_API_URL || "http://localhost:8080";

App(baseUrl);
