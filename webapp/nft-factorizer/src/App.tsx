import React, { useState } from 'react';
import './App.css';
import { Connection, PublicKey, SystemProgram } from '@solana/web3.js';
import { Program, Provider } from '@coral-xyz/anchor';
import idl from './group8.json';

function App() {
  const testLockNFT = async () => {
    // Your TypeScript code for testing lockNFT functionality goes here
  };

  return (
    <div className="App">
      <header className="App-header">
        <button onClick={testLockNFT}>Run LockNFT Test</button>
      </header>
    </div>
  );
}

export default App;