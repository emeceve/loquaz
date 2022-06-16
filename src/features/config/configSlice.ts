import { createAsyncThunk, createSlice, PayloadAction } from "@reduxjs/toolkit";
import * as configService from "../../services/config";
import { Contact } from "../../services/config";
import { RootState } from "../../store";

export interface ConfigState {
  relays: string[];
  contacts: Contact[];
  keys: Keys;
}

interface Keys {
  sk: string;
  pk: string;
}

const initialState: ConfigState = {
  relays: [],
  contacts: [],
  keys: {
    pk: "",
    sk: "",
  },
};

export const configSlice = createSlice({
  name: "config",
  initialState,
  reducers: {
    updatedConfig: (
      state,
      action: PayloadAction<{
        keys?: Keys;
        contacts?: Contact[];
        relays?: string[];
      }>
    ) => {
      const { keys, contacts, relays } = action.payload;
      if (keys) state.keys = keys;
      if (contacts) state.contacts = contacts;
      if (relays) state.relays = relays;
    },
    resetKeys: (state) => {
      state.keys = { sk: "", pk: "" };
    },
  },
  extraReducers: (builder) => {
    builder.addCase(loadConfig.fulfilled, (state, action) => {
      const { contacts, relays } = action.payload;
      state.contacts = contacts;
      state.relays = relays;
    });
    builder.addCase(restoreKeyPair.fulfilled, (state, action) => {
      const [sk, pk] = action.payload;
      state.keys = { sk, pk };
    });
    builder.addCase(generateKeyPair.fulfilled, (state, action) => {
      const [sk, pk] = action.payload;
      console.log(sk);
      state.keys = { sk, pk };
    });
  },
});

export const restoreKeyPair = createAsyncThunk(
  "config/restoreKeyPair",
  async (sk: string) => {
    return await configService.restoreKeyPair(sk);
  }
);

export const generateKeyPair = createAsyncThunk(
  "config/generateKeyPair",
  async () => {
    return await configService.generateKeyPair();
  }
);

export const loadConfig = createAsyncThunk("config/loadConfig", async () => {
  return await configService.getConfig();
});

export const addContact = createAsyncThunk(
  "config/createContact",
  async (contact: Contact, { dispatch }) => {
    await configService.addContact(contact);
    dispatch(loadConfig());
  }
);

export const removeContact = createAsyncThunk(
  "config/removeContact",
  async (contact: Contact, { dispatch }) => {
    await configService.removeContact(contact);
    dispatch(loadConfig());
  }
);

export const addRelay = createAsyncThunk(
  "config/addRelay",
  async (url: string, { dispatch }) => {
    await configService.addRelay(url);
    dispatch(loadConfig());
  }
);

export const removeRelay = createAsyncThunk(
  "config/removeRelay",
  async (url: string, { dispatch }) => {
    await configService.removeRelay(url);
    dispatch(loadConfig());
  }
);

export const selectRelays = (state: RootState) => state.config.relays;
export const selectKeys = (state: RootState) => state.config.keys;
export const selectContacts = (state: RootState) => state.config.contacts;

export const { updatedConfig, resetKeys } = configSlice.actions;
export default configSlice.reducer;
