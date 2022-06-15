import { createAsyncThunk, createSlice, PayloadAction } from "@reduxjs/toolkit";
import {
  Conversation,
  getConversation,
  sendMsg,
  Message,
  MessageSource,
} from "../../services/chat";
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
    receivedNewMessage: (state, action: PayloadAction<Message>) => {
      const { contact, messages } = state.currentConversation;
      const message = action.payload;

      if (
        message.source == MessageSource.THEM &&
        message.ev.pubkey != contact.pk
      )
        return;

      state.currentConversation.messages = [...messages, message];
    },
  },
  extraReducers: (builder) => {
    builder.addCase(selectConversation.fulfilled, (state, action) => {
      state.currentConversation = action.payload;
    });

    builder.addCase(sendMessage.fulfilled, (state, action) => {});
  },
});

export const selectConversation = createAsyncThunk(
  "chat/selectConversation",
  async (pk: string) => {
    return await getConversation(pk);
  }
);

export const sendMessage = createAsyncThunk(
  "chat/sendMsg",
  async ({ pk, content }: { pk: string; content: string }) => {
    return await sendMsg(pk, content);
  }
);

export const selectCurrentConversation = (state: RootState) =>
  state.chat.currentConversation;
export const selectCurrentContact = (state: RootState) =>
  state.chat.currentConversation.contact;
export const { selectedConversation, receivedNewMessage } = chatSlice.actions;
export default chatSlice.reducer;
