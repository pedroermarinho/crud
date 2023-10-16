package com.example.repositories;

import com.example.models.UserEntity;
import jakarta.enterprise.context.ApplicationScoped;

import java.util.List;
import java.util.Optional;

@ApplicationScoped
public class UserRepository {

    public UserEntity save(UserEntity user){
        user.persist();
        return user;
    }

    public Optional<UserEntity> findById(Long id){
        return UserEntity.findByIdOptional(id);
    }

    public List<UserEntity> findAll(){
        return UserEntity.listAll();
    }

    public void delete(Long id){
        this.findById(id).ifPresent(UserEntity::delete);
    }
}
