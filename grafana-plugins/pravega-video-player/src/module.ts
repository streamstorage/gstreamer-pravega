import { PanelPlugin } from '@grafana/data';
// import { LegendDisplayMode, GraphGradientMode } from '@grafana/schema';
import { VideoPlayerOptions } from './types';
import { VideoPlayerPanel } from './VideoPlayerPanel';

export const plugin = new PanelPlugin<VideoPlayerOptions>(VideoPlayerPanel)
  // .useFieldConfig({
  //   standardOptions: {
  //     color: {
  //       defaultValue: {
  //         mode: FieldColorModeId.ContinuousGrYlRd,
  //       },
  //     },
  //   },
  //   useCustomConfig: (builder) => {
  //     builder
  //       .addRadio({
  //         path: 'gradientMode',
  //         name: 'Gradient mode',
  //         defaultValue: GraphGradientMode.Scheme,
  //         settings: {
  //           options: [
  //             {
  //               label: 'None',
  //               value: GraphGradientMode.None,
  //               ariaLabel: ariaLabels.gradientNone,
  //             },
  //             {
  //               label: 'Opacity',
  //               value: GraphGradientMode.Opacity,
  //               description: 'Enable fill opacity gradient',
  //               ariaLabel: ariaLabels.gradientOpacity,
  //             },
  //             {
  //               label: 'Hue',
  //               value: GraphGradientMode.Hue,
  //               description: 'Small color hue gradient',
  //               ariaLabel: ariaLabels.gradientHue,
  //             },
  //             {
  //               label: 'Scheme',
  //               value: GraphGradientMode.Scheme,
  //               description: 'Use color scheme to define gradient',
  //               ariaLabel: ariaLabels.gradientScheme,
  //             },
  //           ],
  //         },
  //       })
  //       .addSliderInput({
  //         path: 'fillOpacity',
  //         name: 'Fill opacity',
  //         defaultValue: 25,
  //         settings: {
  //           min: 0,
  //           max: 100,
  //           step: 1,
  //           ariaLabelForHandle: ariaLabels.fillOpacity,
  //         },
  //       });
  //   },
  // })
  .setPanelOptions((builder) => {
    return builder // .addStringArray({
    //   path: 'params',
    //   name: 'params',
    // });
    .addTextInput({
      path: 'server',
      name: 'Server Prefix',
    })
    .addTextInput({
      path: 'list',
      name: 'Video List',
    });
    // .addTextInput({
    //   path: 'start',
    //   name: 'start',
    // })
    // .addTextInput({
    //   path: 'end',
    //   name: 'end',
    // });
    // .addRadio({
    //   path: 'legend.displayMode',
    //   name: 'Legend mode',
    //   category: ['Legend'],
    //   description: '',
    //   defaultValue: LegendDisplayMode.List,
    //   settings: {
    //     options: [
    //       {
    //         value: LegendDisplayMode.List,
    //         label: 'List',
    //         ariaLabel: ariaLabels.legendDisplayList,
    //       },
    //       {
    //         value: LegendDisplayMode.Table,
    //         label: 'Table',
    //         ariaLabel: ariaLabels.legendDisplayTable,
    //       },
    //       {
    //         value: undefined,
    //         label: 'Hidden',
    //         ariaLabel: ariaLabels.legendDisplayHidden,
    //       },
    //     ],
    //   },
    // })
    // .addRadio({
    //   path: 'legend.placement',
    //   name: 'Legend placement',
    //   category: ['Legend'],
    //   description: '',
    //   defaultValue: 'bottom',
    //   settings: {
    //     options: [
    //       {
    //         value: 'bottom',
    //         label: 'Bottom',
    //         ariaLabel: ariaLabels.legendPlacementBottom,
    //       },
    //       {
    //         value: 'right',
    //         label: 'Right',
    //         ariaLabel: ariaLabels.legendPlacementRight,
    //       },
    //     ],
    //   },
    //   showIf: (config) => !!config.legend.displayMode,
    // });
  });
