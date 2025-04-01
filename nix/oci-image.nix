{
  lib,
  dockerTools,
  main,
  utils ? null,
}:
dockerTools.buildLayeredImage {
  name = main.pname;
  tag = main.version;
  contents = [
    main
  ] ++ lib.optionals (utils != null) [ utils ];

  config =
    {
      cmd = [
        (lib.getExe main)
      ];
    } // lib.optionalAttrs (utils != null) {
      entrypoint = [
        "${utils}/bin/secret-loader"
      ];
    };
}
