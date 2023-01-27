FROM nginx

# Installer OpenSSH-server et OpenSSH-client
RUN apt-get update && apt-get install -y openssh-server openssh-client

# Créer un utilisateur SFTP
RUN useradd -m sftpuser
RUN echo fr00zalgn3t | passwd -d sftpuser

# Exposer les ports SSH et HTTP
EXPOSE 22 80