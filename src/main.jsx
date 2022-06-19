import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';

import { BrowserRouter, Route, Routes } from "react-router-dom";
import App from './routes/App';
import TwoDimensions from './routes/TwoDimensions';
import ThreeDimensions from "./routes/ThreeDimensions";
import Code from "./routes/Code";
import Document from "./routes/Document"
import Addons from "./routes/Addons";

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
    <React.StrictMode>
      <BrowserRouter>
        <Routes>
          <Route path="/" element={<App />}>
            <Route path="/2d" element={<TwoDimensions />} />
            <Route path="/3d" element={<ThreeDimensions />} />
            <Route path="/code" element={<Code />} />
            <Route path="/document" element={<Document />} />
            <Route path="/addons" element={<Addons />} />
          </Route>
        </Routes>
      </BrowserRouter>
    </React.StrictMode>
);