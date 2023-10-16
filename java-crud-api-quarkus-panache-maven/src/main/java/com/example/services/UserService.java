package com.example.services;

import com.example.forms.UserForm;
import com.example.models.UserEntity;
import com.example.repositories.UserRepository;
import jakarta.enterprise.context.ApplicationScoped;
import jakarta.inject.Inject;

import java.util.List;
import java.util.Optional;

@ApplicationScoped
public class UserService {

    @Inject
    UserRepository userRepository;

    public UserEntity create(UserForm form) {
        final UserEntity user = new UserEntity();
        user.setName(form.name());
        user.setEmail(form.email());
        return userRepository.save(user);
    }

    public UserEntity update(Long id, UserForm form) {
        final UserEntity user = UserEntity.findById(id);
        user.setName(form.name());
        user.setEmail(form.email());
        return userRepository.save(user);
    }

    public void delete(Long id) {
        userRepository.delete(id);
    }

    public Optional<UserEntity> findById(Long id) {
        return userRepository.findById(id);
    }

    public List<UserEntity> findAll(){
        return userRepository.findAll();
    }
}
