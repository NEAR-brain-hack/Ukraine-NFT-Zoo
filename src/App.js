import 'regenerator-runtime/runtime'
import React from 'react'
import { login, logout } from './utils'
import './global.css'
import hungryAnimal from './assets/hungry.jpg';
import scaredAnimal from './assets/scared.webp';
import meat from './assets/meat.png'
import vegetable from './assets/vegetable.png'
import zookeeper from './assets/zookeeper.png'
import ticket from './assets/ticket.png'
import { utils, transactions } from "near-api-js";


export default function App() {

  const [totalSupply, setTotalSupply] = React.useState(0)

  React.useEffect(() => {
    window.contract.nft_total_supply().then(supply => setTotalSupply(supply))
  }, [])

  const Donation = {
    Meat: "Meat",
    Vegetable: "Vegetable",
    Ticket: "Ticket",
    ZooKeeper: "ZooKeeper",
  }

  const mint = (type, amount) => {
    window.contract.nft_mint(
      {
        receiver_id: window.accountId,
        donation_type: type
      },
      10000000000000,
      utils.format.parseNearAmount(amount)
    )
  }
  
  if (!window.walletConnection.isSignedIn()) {
    return (
      <main>
        <h1>Welcome to ZOO NFT FOR UKRAINE!</h1>
        <div style={{display: 'flex'}}>
          <div style={{width: "50vw", marginRight: "20px", minWidth: "400px"}}>
            <a href='https://nypost.com/2022/04/06/ukrainian-zoo-will-put-down-thousands-of-animals-after-russian-shelling/' target="_blank">
              <img src={scaredAnimal} style={{width: "100%"}} />
            </a>
          </div>
          <div>
            <p>Volunteers blindfold a pony to reduce its stress levels before taking it to a truck while.
            attempting to evacuate the surviving animals. </p>
            <p>You could see how war effects badly on animals. </p>
          </div>
        </div>
        <div style={{display: 'flex'}}>
          <div>
            <p>Many animals in Ukraine will face starvation because no one takes care of them.</p>
            <p>Our organization is raising funds for Ukrainian zoos. You could donate money spent on animal rescue in Ukraine by minting NFTs. </p>
            <p>Sign in and try it out.</p>
          </div>
          <div style={{width: "40vw", marginRight: "20px", minWidth: "400px"}}>
            <a href='https://nypost.com/2022/04/06/ukrainian-zoo-will-put-down-thousands-of-animals-after-russian-shelling/' target="_blank">
              <img src={hungryAnimal} style={{width: "100%"}} />
            </a>
          </div>
        </div>
        <button className="link" style={{ position: 'fixed', top: '20px', right: '20px' }} onClick={login}>
          Sign in
        </button>
      </main>
    )
  }

  return (
    // use React Fragment, <>, to avoid wrapping elements in unnecessary divs
    <>
      <button className="link" style={{ float: 'right' }} onClick={logout}>
        Sign out
      </button>
      <main>
        <h1>
          Donate to Ukrainian zoos' animals and staff
        </h1>
        <div style={{marginTop: "50px", textAlign: "center"}}>
          <h3>Select which you like to donate to the zoos.</h3>
        </div>
        <div style={{display: 'flex', flex: 'wrap' ,justifyContent: 'space-evenly', marginTop: '30px'}}>
          <div className='donate-select' onClick={() => mint(Donation["Meat"], "4")}>
            <div>
              <img src={meat} />
            </div>
            <div className='title'>
              <h2>MEAT FOR LION</h2>
              <p>4 NEAR would be donated for carnivores</p>
            </div>
          </div>
          <div className='donate-select' onClick={() => mint(Donation["Vegetable"], "2")}>
            <div>
              <img src={vegetable} />
            </div>
            <div className='title'>
              <h2>VEGETABLE FOR RABBIT</h2>
              <p>2 NEAR would be donated for herbivores</p>
            </div>
          </div>
          <div className='donate-select' onClick={() => mint(Donation["ZooKeeper"], "2")}>
            <div>
              <img src={zookeeper} />
            </div>
            <div className='title'>
              <h2>DONATE FOR ZOOKEEPER</h2>
              <p>2 NEAR would be donated for zookeepers</p>
            </div>
          </div>
          <div className='donate-select' onClick={() => mint(Donation["Ticket"], "3")}>
            <div>
              <img src={ticket} />
            </div>
            <div className='title'>
              <h2>BUY TICKET</h2>
              <p>Buy a ticket and visit someday, 3 NEAR for each</p>
            </div>
          </div>
        </div>
        <div style={{marginTop: "50px", textAlign: "center"}}>
          <h3>{totalSupply} was minted, mint more for Ukrainian animals</h3>
        </div>
      </main>
    </>
  )
}


