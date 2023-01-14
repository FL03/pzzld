{ pkgs, rustc, cargo };

{
  packages = [ pkgs.git ];
  
  enterShell = ''
    hello
    git --version
  '';
}