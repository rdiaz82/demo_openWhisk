import 'bootstrap/dist/css/bootstrap.min.css';
import React, {
  Component
} from 'react';
import './App.css';
import logo from "./images/logo.svg";
import Chart from './components/Chart';
import Tanks from './components/tanks';
require('dotenv')
  .config();

class App extends Component {
    render() {

      return (
              <div className = "App" >
              <img className="col-2 logo" src={logo}/>
              <div>
              <Tanks tank1={"sensor1"}
          tank2={"sensor2"}
          valve1={"sensor5"}
          valve2={"sensor6"}
          valve3={"sensor7"}
          pump={"sensor8"}
          heater1={"sensor9"}
          heater2={"sensor10"}
              />
            </div>
      <div class ="row align-items-center">
      <div className ="col-6">
              <Chart sensor = {"sensor3"} label={"Mashing Tank"} />
            </div>
              <div className ="col-6">
              <Chart sensor = {"sensor4"} label={"Boiling Tank"}/>
            </div>
                </div>
                </div>
    );
    }
}

export default App;
