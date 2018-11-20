import React, {
  Component
} from 'react';
import {
  color
} from 'd3-color';
import {
  interpolateRgb
} from 'd3-interpolate';
import LiquidFillGauge from 'react-liquid-gauge';
import * as RealtimeMessaging from 'realtime-messaging';
import dotenv from 'dotenv';

class Gauge extends Component {
  state = {
    value: 0
  };
  startColor = '#FFD923';
  endColor = '#FFD923';

  componentDidMount() {
    dotenv.config();
    const client = RealtimeMessaging.createClient();
    client.setClusterUrl("http://ortc-developers.realtime.co/server/2.1/");
    client.connect(process.env.REACT_APP_RT_APIKEY, process.env.REACT_APP_RT_SECRET);
    client.onConnected = (client) => {
      console.log("realtime connected");
      client.subscribe(this.props.sensor, true, (client, channel, message) => {
        console.log("Received message:", message);
        const jsonObj = JSON.parse(message);
        this.setState({
          value: jsonObj.measurement
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
            return Date.parse(b.timestamp) - Date.parse(a.timestamp);
          });
          console.log(arrObj);
          if (arrObj.length > 0) {
            this.setState({
              value: responseData.result[0].measurement
            });
          }
        }

      });
  }

  render() {
    const radius = 40;
    const interpolate = interpolateRgb(this.startColor, this.endColor);
    const fillColor = interpolate(this.state.value / 100);
    const gradientStops = [{
        key: '0%',
        stopColor: color(fillColor)
          .darker(0.5)
          .toString(),
        stopOpacity: 1,
        offset: '0%'
      },
      {
        key: '50%',
        stopColor: fillColor,
        stopOpacity: 0.75,
        offset: '50%'
      },
      {
        key: '100%',
        stopColor: color(fillColor)
          .brighter(0.5)
          .toString(),
        stopOpacity: 0.5,
        offset: '100%'
      }
    ];

    return (
      <div>
                <LiquidFillGauge
                    style={{ margin: '0 auto' }}
                    width={radius * 2}
                    height={radius * 2}
                    value={this.state.value}
                    percent="%"
                    textSize={1}
                    textOffsetX={0}
                    textOffsetY={0}
                    textRenderer={(props) => {
                        const value = Math.round(props.value);
                        const radius = Math.min(props.height / 2, props.width / 2);
                        const textPixels = (props.textSize * radius / 2);
                        const valueStyle = {
                            fontSize: textPixels
                        };
                        const percentStyle = {
                            fontSize: textPixels * 0.6
                        };
 
                        return (
                            <tspan>
                                <tspan className="value" style={valueStyle}>{value}</tspan>
                                <tspan style={percentStyle}>{props.percent}</tspan>
                            </tspan>
                        );
                    }}
                    riseAnimation
                    waveAnimation
                    waveFrequency={2}
                    waveAmplitude={2}
                    gradient
                    gradientStops={gradientStops}
                    circleStyle={{
                        fill: fillColor
                    }}
                    waveStyle={{
                        fill: fillColor
                    }}
                    textStyle={{
                        fill: color('#fff').toString(),
                        fontFamily: 'Arial'
                    }}
                    waveTextStyle={{
                        fill: color('#fff').toString(),
                        fontFamily: 'Arial'
                    }}
                />
            </div>
    );
  }
}

export default Gauge;
