import React from 'react'
import ReactDOM from 'react-dom'
import './index.css'
import App from './App'
import {invoke} from '@tauri-apps/api/tauri';
import {listen} from '@tauri-apps/api/event';
import {BrowserRouter} from 'react-router-dom';

 

ReactDOM.render(
  <React.StrictMode>
    <BrowserRouter>
      <App />
    </BrowserRouter>
  </React.StrictMode>,
  document.getElementById('root')
)
