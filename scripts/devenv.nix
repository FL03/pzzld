{ pkgs, ... };

{
  packages = [ pkgs.git ];
  
  enterShell = ''
    hello
    git --version
  '';
}