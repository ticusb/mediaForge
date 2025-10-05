
import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css'; // Optional: for global styles
import App from './App.jsx';
import reportWebVitals from './reportWebVitals'; // Optional: for performance metrics

// // MediaForge frontend placeholder
// console.log('MediaForge frontend placeholder - JS stack ready');

// (function(){
// 	const e = React.createElement;
// 	// Wait for components to be available on window
// 	function mount(){
// 		if(!window.Dashboard) return setTimeout(mount, 50);
// 		const root = document.getElementById('root');
// 		const Dashboard = window.Dashboard.Dashboard;
// 		ReactDOM.createRoot(root).render(e(Dashboard));
// 	}

// 	mount();
// })();

// index.js

const root = ReactDOM.createRoot(document.getElementById('root'));
root.render(
    <React.StrictMode>
        <App />
    </React.StrictMode>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();