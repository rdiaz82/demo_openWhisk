import React, {Component} from 'react';
import Gauge from './Gauge';
import ImgSwitch from './imageSwitch';
import valveOn from '../images/valve_on.png';
import valveOff from '../images/valve_off.png';
import pumpOff from '../images/pump_off.png';
import pumpOn from '../images/pump_on.png';
import heaterOff from '../images/heat_off.png';
import heaterOn from '../images/heat_on.png';



class Tanks extends Component {
    render(){
        return (
                <div className="tankImage">
                <div className="tank1Gauge">
                <Gauge sensor={this.props.tank1}/>
                </div>
                <ImgSwitch sensor={this.props.valve1} img_off={valveOff} img_on={valveOn} class={"valve1"}/>
                <ImgSwitch sensor={this.props.valve2} img_off={valveOff} img_on={valveOn} class={"valve2"}/>
                <ImgSwitch sensor={this.props.valve3} img_off={valveOff} img_on={valveOn} class={"valve3"}/>
                <ImgSwitch sensor={this.props.pump} img_off={pumpOff} img_on={pumpOn} class={"pump"}/>
                <ImgSwitch sensor={this.props.heater1} img_off={heaterOff} img_on={heaterOn} class={"heater1"}/>
                <ImgSwitch sensor={this.props.heater2} img_off={heaterOff} img_on={heaterOn} class={"heater2"}/>
                <div className="tank2Gauge">
                <Gauge  sensor={this.props.tank2}/>
                </div>
            </div>
        );
    }

 
}

export default Tanks;
