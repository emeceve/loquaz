import { configureStore } from "@reduxjs/toolkit";
import configReducer from "./features/config/configSlice";
import chatReducer from "./features/chat/chatSlice";

export const store = configureStore({
  reducer: {
    config: configReducer,
    chat: chatReducer,
  },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;
