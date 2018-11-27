import React, {
  Component
} from 'react';
import {
  Line
} from 'react-chartjs-2';

import {
  defaults
} from 'react-chartjs-2';
import * as RealtimeMessaging from 'realtime-messaging';
import dotenv from 'dotenv';

// Disable animating charts by default.
defaults.global.animation = false;
defaults.global.legend = false;

class Chart extends Component {

  state = {
    displayTitle: false,
      displayLegend: false,
      displayLabel: false,
    data: {
      datasets: [{
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
      }]
    },
    options: {
      scales: {
        xAxes: [{
          type: 'time',
          time: {
            unit: 'minute'
          },
          scaleLabel: {
            display: true,
            labelString: 'Date'
          }
        }],
        yAxes: [{
          scaleLabel: {
            display: true,
            labelString: 'Temperature (ºC)'
          }
        }]
      },
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
      client.subscribe(this.props.sensor, true, (client, channel, message) => {
        console.log("Received message:", message);
        const jsonObj = JSON.parse(message);
        const datasetsCopy = this.state.data.datasets.slice(0);
        const timestamps = this.state.data.labels.slice(0);
        timestamps.push(jsonObj.timestamp);
        datasetsCopy[0].data.push(jsonObj.measurement);
        if (datasetsCopy[0].data.length > 50) {
          datasetsCopy[0].data.shift();
          timestamps.shift();
        }
        this.setState({
          data: Object.assign({}, this.state.data, {
            datasets: datasetsCopy,
            labels: timestamps
          })
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
          console.log(responseData.result);
          if (responseData.result != null) {
        const arrObj = responseData.result.sort(function(a, b) {
          return Date.parse(a.timestamp) - Date.parse(b.timestamp);
        });
          
        console.log(arrObj);
        responseData.result.forEach(function(item) {
          measurements.push(item.measurement);
          timestamps.push(item.timestamp);
        });

        const datasetsCopy = this.state.data.datasets.slice(0);
        datasetsCopy[0].data = measurements;
        this.setState({
          data: Object.assign({}, this.state.data, {
            datasets: datasetsCopy,
            labels: timestamps
          })
        });
          }
      });
  }

  render() {
      return (
<div>
              <p><span class="dot"></span> {this.props.label}</p>
              < div className = "chart" >

      <
      Line data = {
        this.state.data
      }
      options = {
        this.state.options
      }
      width = {
        this.props.width
      }
      height = {
        this.props.height
      }
      /></div ></div>
    );
  }
}

export default Chart;
