import React, {
    Component
} from 'react';
import * as RealtimeMessaging from 'realtime-messaging';
import dotenv from 'dotenv';
import valveOn from '../images/valve_on.png';


class ImgSwitch extends Component {
    constructor(props) {
        super(props);
        this.state = { status: props.img_off };
    }
    componentDidMount() {
        dotenv.config();
        const client = RealtimeMessaging.createClient();
        client.setClusterUrl("http://ortc-developers.realtime.co/server/2.1/");
        client.connect(process.env.REACT_APP_RT_APIKEY, process.env.REACT_APP_RT_SECRET);
        client.onConnected = (client) => {
            client.subscribe (this.props.sensor, true, (client, channel, message) => {
                const jsonObj = JSON.parse(message);
                this.setState({
                    status: jsonObj.measurement==0?this.props.img_off:this.props.img_on
                });
            });
        };
        var measurements = [];
        var timestamps = [];

        fetch('/api/v1/web/rdiaz82IotDemo_dev/default/fetchData.json', {
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json'
                },
                method: 'post',
                body: JSON.stringify({
                    'sensor': this.props.sensor
                })
            })
            .then(function(res) {
                return res.json();
            })
            .then((responseData) => {
                if(responseData.result != null){
                const arrObj = responseData.result.sort(function(a, b) {
                    return Date.parse(b.timestamp) - Date.parse(a.timestamp);
                });
                    if (arrObj.length>0){
                        console.log(arrObj[0].measurement==0?this.props.img_off:this.props.img_on);
                this.setState({
                    status: arrObj[0].measurement==0?this.props.img_off:this.props.img_on
                }
                );
                }}

            });
    }
    
    render() {
        return(<div className={this.props.class}> <img src={this.state.status}/> </div>);
    }

}

export default ImgSwitch;
