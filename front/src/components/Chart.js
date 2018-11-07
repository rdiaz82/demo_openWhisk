import React, {Component} from 'react';
import {Bar, Line, Pie} from 'react-chartjs-2';

import { defaults } from 'react-chartjs-2';
import * as RealtimeMessaging from 'realtime-messaging';
import dotenv from 'dotenv';


// Disable animating charts by default.
defaults.global.animation = false;

class Chart extends Component{

  state = {
    displayTitle:true,
    displayLegend: true,
      legendPosition:'right',
      data : {
          labels: ['0', '1', '2', '3', '4', '5', '6'],
          datasets: [
              {
                  label: 'My First dataset',
                  fill: false,
                  lineTension: 0.1,
                  backgroundColor: 'rgba(75,192,192,0.4)',
                  borderColor: 'rgba(75,192,192,1)',
                  borderCapStyle: 'butt',
                  borderDash: [],
                  borderDashOffset: 0.0,
                  borderJoinStyle: 'miter',
                  pointBorderColor: 'rgba(75,192,192,1)',
                  pointBackgroundColor: '#fff',
                  pointBorderWidth: 1,
                  pointHoverRadius: 5,
                  pointHoverBackgroundColor: 'rgba(75,192,192,1)',
                  pointHoverBorderColor: 'rgba(220,220,220,1)',
                  pointHoverBorderWidth: 2,
                  pointRadius: 1,
                  pointHitRadius: 10,
                  data: [65, 59, 80, 81, 56, 55, 40]
              }
          ]
      }

  }

    componentDidMount() {
        dotenv.config();
        console.log(process.env.REACT_APP_RT_APIKEY);
        const client = RealtimeMessaging.createClient();
        client.setClusterUrl("http://ortc-developers.realtime.co/server/2.1/");
        client.connect(process.env.REACT_APP_RT_APIKEY, process.env.REACT_APP_RT_SECRET);
        client.onConnected = (client) => {
            console.log("realtime connected");
            client.subscribe("channel", true, (client, channel, message) => {
               console.log("Received message:", message);
            });
        }
    }

    componentWillUnmount() {
        clearInterval(this.timer)
        
    }

    increment() {
        const datasetsCopy = this.state.data.datasets.slice(0);
        const labelCopy = this.state.data.labels.slice(0);
        labelCopy.push((parseInt(labelCopy[labelCopy.length-1])+1).toString());
        const dataCopy = datasetsCopy[0].data.slice(0);
        dataCopy.push(dataCopy[0]);
        dataCopy.splice(0,1);
        datasetsCopy[0].data = dataCopy;

       
        labelCopy.splice(0, 1);

        this.setState({
            data: Object.assign({}, this.state.data, {
                datasets: datasetsCopy,
                labels: labelCopy
            })
        });
    }



  render(){
      return (
          <div classname="chart">
        <Line
          data={this.state.data}
        />
      </div>
      )
  }
}

export default Chart;
