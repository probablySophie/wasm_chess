tmux clear-history; 
clear; 
if . build.sh; then
	printf "${green}Successfully built!${normal}\n"
	http www -q
else
	printf "${red}Failed to build.  Not running web server${normal}\n"
fi
