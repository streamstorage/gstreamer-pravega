import React from 'react';
import { PanelProps } from '@grafana/data';
// import { TimeSeries, TooltipPlugin, TooltipDisplayMode, ZoomPlugin } from '@grafana/ui';
import { VideoPlayerOptions } from './types';

interface Props extends PanelProps<VideoPlayerOptions> {}

export function VideoPlayerPanel({
  options,
  data,
  width,
  height,
  timeZone,
  timeRange,
  onChangeTimeRange,
  replaceVariables,
}: Props) {
  // console.log('Panel rendered. ✔️');

  // const [params, setParams] = React.useState([] as string[][]);
  // React.useEffect(
  //   () => {
  //     setParams(
  //       replaceVariables('$params')
  //         .split('|')
  //         .map((row) => row.split(' '))
  //     );
  //     console.log(`useEffect: ${replaceVariables('$params')}`);
  //   },
  //   /* eslint-disable */
  //   [replaceVariables, replaceVariables('$params')]
  // );

  console.log(`params: ${replaceVariables('$params')}`);
  const params = replaceVariables('$params')
    .split('|')
    .map((row) => row.split(' '));

  console.log(params.map((row) => `${options.server}/player?scope=${row[0]}&stream=${row[1]}&begin=${row[2]}&end=${row[3]}`));

  return (
    <div data-testid="basic-panel-example" style={{ height: '100%' }}>
      {params.map((row) => (
        <iframe
          key={row.join(' ')}
          allowFullScreen
          height="50%"
          width="33%"
          src={`${options.server}/player?scope=${row[0]}&stream=${row[1]}&begin=${row[2]}&end=${row[3]}`}
        />
      ))}
    </div>
  );
}
