import { createAsyncThunk, createSlice, PayloadAction } from "@reduxjs/toolkit";
import { Conversation, getConversation } from "../../services/chat";
import { RootState } from "../../store";

export interface ChatState {
  currentConversation: Conversation;
}

const initialState: ChatState = {
  currentConversation: { contact: { alias: "", pk: "" }, messages: [] },
};

export const chatSlice = createSlice({
  name: "chat",
  initialState,
  reducers: {
    selectedConversation: (state, action: PayloadAction<Conversation>) => {
      state.currentConversation = action.payload;
    },
  },
  extraReducers: (builder) => {
    builder.addCase(selectConversation.fulfilled, (state, action) => {
      state.currentConversation = action.payload;
    });
  },
});

export const selectConversation = createAsyncThunk(
  "chat/selectConversation",
  async (pk: string) => {
    return await getConversation(pk);
  }
);

export const selectCurrentConversation = (state: RootState) =>
  state.chat.currentConversation;
export const selectCurrentContact = (state: RootState) =>
  state.chat.currentConversation.contact;
export const { selectedConversation } = chatSlice.actions;
export default chatSlice.reducer;
